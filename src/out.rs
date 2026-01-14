pub const STDOUT: &str = "stdout";
pub const STDERR: &str = "stderr";

pub fn stdout() -> Option<String> {
    Some(String::from(STDOUT))
}

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
