/// Convenience macros for creating alerts

/// Creates a trace alert
#[macro_export]
macro_rules! trace {
    ($msg:expr) => {
        $crate::Alert::trace($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Alert::trace(format!($fmt, $($arg)*))
    };
}

/// Creates a debug alert
#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        $crate::Alert::debug($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Alert::debug(format!($fmt, $($arg)*))
    };
}

/// Creates an info alert
#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        $crate::Alert::info($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Alert::info(format!($fmt, $($arg)*))
    };
}

/// Creates a warning alert
#[macro_export]
macro_rules! warning {
    ($msg:expr) => {
        $crate::Alert::warning($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Alert::warning(format!($fmt, $($arg)*))
    };
}

/// Creates an error alert
#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        $crate::Alert::error($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Alert::error(format!($fmt, $($arg)*))
    };
}

/// Creates a critical alert
#[macro_export]
macro_rules! critical {
    ($msg:expr) => {
        $crate::Alert::critical($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Alert::critical(format!($fmt, $($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use crate::types::{Alert, AlertLevel};

    #[test]
    fn test_info_macro() {
        let alert = info!("Test message");
        assert_eq!(alert.level, AlertLevel::Info);
        assert_eq!(alert.message, "Test message");
    }

    #[test]
    fn test_error_macro_with_format() {
        let value = 42;
        let alert = error!("Error code: {}", value);
        assert_eq!(alert.level, AlertLevel::Error);
        assert_eq!(alert.message, "Error code: 42");
    }
}
