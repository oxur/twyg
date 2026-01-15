//! Backwards compatibility constants for output destinations.
//!
//! This module provides legacy string-based constants and functions for
//! specifying output destinations. New code should use the [`Output`](crate::Output)
//! enum instead.
//!
//! # Examples
//!
//! ```
//! use twyg::{STDOUT, STDERR};
//!
//! assert_eq!(STDOUT, "stdout");
//! assert_eq!(STDERR, "stderr");
//! ```

/// String constant for standard output destination.
///
/// Consider using [`Output::Stdout`](crate::Output::Stdout) instead for type safety.
pub const STDOUT: &str = "stdout";

/// String constant for standard error destination.
///
/// Consider using [`Output::Stderr`](crate::Output::Stderr) instead for type safety.
pub const STDERR: &str = "stderr";

/// Returns "stdout" as an Option<String> for backwards compatibility.
///
/// Consider using [`Output::Stdout`](crate::Output::Stdout) instead.
pub fn stdout() -> Option<String> {
    Some(String::from(STDOUT))
}

/// Returns "stderr" as an Option<String> for backwards compatibility.
///
/// Consider using [`Output::Stderr`](crate::Output::Stderr) instead.
pub fn stderr() -> Option<String> {
    Some(String::from(STDERR))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(STDOUT, "stdout");
        assert_eq!(STDERR, "stderr");
    }

    #[test]
    fn test_stdout_returns_some() {
        let result = stdout();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "stdout");
    }

    #[test]
    fn test_stderr_returns_some() {
        let result = stderr();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "stderr");
    }

    #[test]
    fn test_stdout_allocates_new_string() {
        let s1 = stdout().unwrap();
        let s2 = stdout().unwrap();
        // Each call allocates a new String
        assert_eq!(s1, s2);
    }
}
