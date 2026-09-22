//! Darwin pthread quality-of-service binding.

use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[error("pthread_set_qos_class_self_np failed with errno {code}")]
pub struct QosError {
    pub code: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct QosClass(pub u32);

pub const QOS_CLASS_USER_INTERACTIVE: QosClass = QosClass(0x21);

pub trait QosSetter {
    /// Applies a `QoS` class and relative priority to the calling thread.
    ///
    /// # Errors
    ///
    /// Returns the pthread error number when the scheduler rejects the request.
    fn set_current(&self, class: QosClass, relative_priority: i32) -> Result<(), QosError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct NativeQosSetter;

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn pthread_set_qos_class_self_np(qos_class: u32, relative_priority: i32) -> i32;
}

#[cfg(target_os = "macos")]
impl QosSetter for NativeQosSetter {
    fn set_current(&self, class: QosClass, relative_priority: i32) -> Result<(), QosError> {
        // SAFETY: the function affects only the calling thread and accepts scalar values.
        let code = unsafe { pthread_set_qos_class_self_np(class.0, relative_priority) };
        if code == 0 {
            Ok(())
        } else {
            Err(QosError { code })
        }
    }
}

/// Marks the calling inference thread as latency-sensitive.
///
/// This is a scheduler hint; Darwin does not expose hard P-core affinity.
///
/// # Errors
///
/// Returns an error when pthread rejects the `QoS` class.
pub fn bind_inference_thread(setter: &impl QosSetter) -> Result<(), QosError> {
    setter.set_current(QOS_CLASS_USER_INTERACTIVE, 0)
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::{QOS_CLASS_USER_INTERACTIVE, QosClass, QosError, QosSetter, bind_inference_thread};

    struct Recorder(Cell<Option<(QosClass, i32)>>);

    impl QosSetter for Recorder {
        fn set_current(&self, class: QosClass, priority: i32) -> Result<(), QosError> {
            self.0.set(Some((class, priority)));
            Ok(())
        }
    }

    #[test]
    fn inference_binding_uses_interactive_zero_priority() {
        let recorder = Recorder(Cell::new(None));
        bind_inference_thread(&recorder).unwrap();
        assert_eq!(recorder.0.get(), Some((QOS_CLASS_USER_INTERACTIVE, 0)));
    }

    #[test]
    fn interactive_class_matches_darwin_encoding() {
        assert_eq!(QOS_CLASS_USER_INTERACTIVE.0, 0x21);
    }

    #[test]
    fn error_display_includes_errno() {
        assert_eq!(
            QosError { code: 22 }.to_string(),
            "pthread_set_qos_class_self_np failed with errno 22"
        );
    }
}
