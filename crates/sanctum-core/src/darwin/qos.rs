//! Darwin pthread quality-of-service binding.

use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[error("pthread_set_qos_class_self_np failed with errno {code}")]
pub struct QosError {
    pub code: i32,
}

#[cfg(test)]
mod tests {
    use super::QosError;

    #[test]
    fn error_display_includes_errno() {
        assert_eq!(
            QosError { code: 22 }.to_string(),
            "pthread_set_qos_class_self_np failed with errno 22"
        );
    }
}
