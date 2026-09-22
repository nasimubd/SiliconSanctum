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

#[cfg(test)]
mod tests {
    use super::{QOS_CLASS_USER_INTERACTIVE, QosError};

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
