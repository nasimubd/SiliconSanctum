//! Grand Central Dispatch memory-pressure monitoring.

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryPressure {
    Normal,
    Warning,
    Critical,
}

pub const PRESSURE_NORMAL: usize = 0x01;
pub const PRESSURE_WARNING: usize = 0x02;
pub const PRESSURE_CRITICAL: usize = 0x04;

#[must_use]
pub const fn decode_pressure(flags: usize) -> MemoryPressure {
    if flags & PRESSURE_CRITICAL != 0 {
        MemoryPressure::Critical
    } else if flags & PRESSURE_WARNING != 0 {
        MemoryPressure::Warning
    } else {
        MemoryPressure::Normal
    }
}

pub trait PressureHandler: Send + Sync + 'static {
    fn on_pressure(&self, pressure: MemoryPressure);
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum PressureMonitorError {
    #[error("dispatch failed to create a memory-pressure source")]
    SourceCreation,
}

pub fn dispatch_pressure(handler: &impl PressureHandler, flags: usize) {
    handler.on_pressure(decode_pressure(flags));
}

#[cfg(target_os = "macos")]
mod native {
    use std::ffi::{c_long, c_ulong, c_void};
    use std::ptr::NonNull;

    use super::{
        PRESSURE_CRITICAL, PRESSURE_NORMAL, PRESSURE_WARNING, PressureHandler,
        PressureMonitorError, dispatch_pressure,
    };

    type DispatchObject = *mut c_void;
    type DispatchFunction = unsafe extern "C" fn(*mut c_void);

    unsafe extern "C" {
        static _dispatch_source_type_memorypressure: c_void;
        fn dispatch_get_global_queue(identifier: c_long, flags: c_ulong) -> DispatchObject;
        fn dispatch_source_create(
            kind: *const c_void,
            handle: usize,
            mask: usize,
            queue: DispatchObject,
        ) -> DispatchObject;
        fn dispatch_source_get_data(source: DispatchObject) -> usize;
        fn dispatch_set_context(object: DispatchObject, context: *mut c_void);
        fn dispatch_set_finalizer_f(object: DispatchObject, finalizer: DispatchFunction);
        fn dispatch_source_set_event_handler_f(source: DispatchObject, handler: DispatchFunction);
        fn dispatch_activate(object: DispatchObject);
        fn dispatch_source_cancel(source: DispatchObject);
        fn dispatch_release(object: DispatchObject);
    }

    struct CallbackContext {
        source: DispatchObject,
        handler: Box<dyn PressureHandler>,
    }

    unsafe extern "C" fn event_handler(context: *mut c_void) {
        // SAFETY: dispatch invokes this only with the context installed below.
        let context = unsafe { &*context.cast::<CallbackContext>() };
        // SAFETY: the source remains retained until after cancellation and finalization.
        let flags = unsafe { dispatch_source_get_data(context.source) };
        dispatch_pressure(&context.handler, flags);
    }

    unsafe extern "C" fn finalize_context(context: *mut c_void) {
        // SAFETY: this reconstructs the single Box transferred to dispatch.
        drop(unsafe { Box::from_raw(context.cast::<CallbackContext>()) });
    }

    impl PressureHandler for Box<dyn PressureHandler> {
        fn on_pressure(&self, pressure: super::MemoryPressure) {
            self.as_ref().on_pressure(pressure);
        }
    }

    pub struct MemoryPressureMonitor {
        source: NonNull<c_void>,
    }

    impl MemoryPressureMonitor {
        /// Creates and activates a GCD memory-pressure source.
        ///
        /// # Errors
        ///
        /// Returns an error when libdispatch cannot allocate the source.
        pub fn start(handler: impl PressureHandler) -> Result<Self, PressureMonitorError> {
            // SAFETY: the source type and global queue are process-lifetime libdispatch objects.
            let source = unsafe {
                dispatch_source_create(
                    &raw const _dispatch_source_type_memorypressure,
                    0,
                    PRESSURE_NORMAL | PRESSURE_WARNING | PRESSURE_CRITICAL,
                    dispatch_get_global_queue(0, 0),
                )
            };
            let source = NonNull::new(source).ok_or(PressureMonitorError::SourceCreation)?;
            let context = Box::into_raw(Box::new(CallbackContext {
                source: source.as_ptr(),
                handler: Box::new(handler),
            }));
            // SAFETY: source is owned by this monitor and context is released by its finalizer.
            unsafe {
                dispatch_set_context(source.as_ptr(), context.cast());
                dispatch_set_finalizer_f(source.as_ptr(), finalize_context);
                dispatch_source_set_event_handler_f(source.as_ptr(), event_handler);
                dispatch_activate(source.as_ptr());
            }
            Ok(Self { source })
        }

        pub fn cancel(&self) {
            // SAFETY: source is valid for the monitor lifetime and cancellation is idempotent.
            unsafe { dispatch_source_cancel(self.source.as_ptr()) };
        }
    }

    impl Drop for MemoryPressureMonitor {
        fn drop(&mut self) {
            self.cancel();
            // SAFETY: this releases the monitor's single owning reference.
            unsafe { dispatch_release(self.source.as_ptr()) };
        }
    }
}

#[cfg(target_os = "macos")]
pub use native::MemoryPressureMonitor;

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::{
        MemoryPressure, PRESSURE_CRITICAL, PRESSURE_NORMAL, PRESSURE_WARNING, PressureHandler,
        decode_pressure, dispatch_pressure,
    };

    #[derive(Default)]
    struct Recorder(Mutex<Vec<MemoryPressure>>);

    impl PressureHandler for Recorder {
        fn on_pressure(&self, pressure: MemoryPressure) {
            self.0.lock().unwrap().push(pressure);
        }
    }

    #[test]
    fn invokes_warning_callback() {
        let recorder = Recorder::default();
        dispatch_pressure(&recorder, PRESSURE_WARNING);
        assert_eq!(*recorder.0.lock().unwrap(), [MemoryPressure::Warning]);
    }

    #[test]
    fn invokes_critical_callback() {
        let recorder = Recorder::default();
        dispatch_pressure(&recorder, PRESSURE_CRITICAL);
        assert_eq!(*recorder.0.lock().unwrap(), [MemoryPressure::Critical]);
    }

    #[test]
    fn decodes_normal_event() {
        assert_eq!(decode_pressure(PRESSURE_NORMAL), MemoryPressure::Normal);
    }

    #[test]
    fn decodes_warning_event() {
        assert_eq!(decode_pressure(PRESSURE_WARNING), MemoryPressure::Warning);
    }

    #[test]
    fn decodes_critical_event() {
        assert_eq!(decode_pressure(PRESSURE_CRITICAL), MemoryPressure::Critical);
    }
}
