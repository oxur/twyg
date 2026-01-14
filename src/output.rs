//! Output destination types and conversions.
//!
//! This module provides the [`Output`] enum for type-safe output destination configuration.

use owo_colors::Stream;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Output destination for log messages.
///
/// Specifies where log messages should be written: standard output,
/// standard error, or a file.
///
/// # Examples
///
/// ```
/// use twyg::Output;
///
/// let stdout = Output::Stdout;
/// let stderr = Output::Stderr;
/// let file = Output::file("/var/log/app.log");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Output {
    /// Write to standard output (stdout).
    Stdout,
    /// Write to standard error (stderr).
    Stderr,
    /// Write to a file at the specified path.
    File(PathBuf),
}

impl Output {
    /// Creates a new file output destination.
    ///
    /// # Examples
    ///
    /// ```
    /// use twyg::Output;
    ///
    /// let output = Output::file("/var/log/app.log");
    /// ```
    pub fn file<P: AsRef<Path>>(path: P) -> Self {
        Output::File(path.as_ref().to_path_buf())
    }

    /// Returns the string representation for backwards compatibility.
    pub fn as_str(&self) -> &str {
        match self {
            Output::Stdout => "stdout",
            Output::Stderr => "stderr",
            Output::File(_) => "file",
        }
    }

    /// Returns true if this output is to a file.
    pub fn is_file(&self) -> bool {
        matches!(self, Output::File(_))
    }

    /// Returns the file path if this is a file output.
    pub fn file_path(&self) -> Option<&Path> {
        match self {
            Output::File(path) => Some(path),
            _ => None,
        }
    }
}

impl Default for Output {
    /// Returns the default output: [`Output::Stdout`].
    fn default() -> Self {
        Self::Stdout
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Output::Stdout => write!(f, "stdout"),
            Output::Stderr => write!(f, "stderr"),
            Output::File(path) => write!(f, "file:{}", path.display()),
        }
    }
}

impl FromStr for Output {
    type Err = ParseOutputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stdout" => Ok(Output::Stdout),
            "stderr" => Ok(Output::Stderr),
            _ if s.starts_with("file:") => {
                let path = &s[5..];
                Ok(Output::File(PathBuf::from(path)))
            }
            _ => {
                // Assume it's a file path
                Ok(Output::File(PathBuf::from(s)))
            }
        }
    }
}

/// Error returned when parsing an output destination fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseOutputError {
    invalid_input: String,
}

impl fmt::Display for ParseOutputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid output destination '{}', expected: stdout, stderr, or a file path",
            self.invalid_input
        )
    }
}

impl std::error::Error for ParseOutputError {}

/// Convert Output to owo_colors' Stream for colored output.
impl From<&Output> for Stream {
    fn from(output: &Output) -> Self {
        match output {
            Output::Stdout | Output::File(_) => Stream::Stdout,
            Output::Stderr => Stream::Stderr,
        }
    }
}

// Backwards compatibility module
pub mod compat {
    /// Constant for stdout (backwards compatibility).
    pub const STDOUT: &str = "stdout";

    /// Constant for stderr (backwards compatibility).
    pub const STDERR: &str = "stderr";

    /// Returns stdout as an Option<String> (backwards compatibility).
    #[deprecated(since = "0.6.0", note = "Use Output::Stdout instead")]
    pub fn stdout() -> Option<String> {
        Some(String::from(STDOUT))
    }

    /// Returns stderr as an Option<String> (backwards compatibility).
    #[deprecated(since = "0.6.0", note = "Use Output::Stderr instead")]
    pub fn stderr() -> Option<String> {
        Some(String::from(STDERR))
    }
}

// Re-export for backwards compatibility
pub use compat::{STDERR, STDOUT};

#[cfg(test)]
mod tests {
    use super::{compat, Output, Stream, STDERR, STDOUT};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_output_default() {
        assert_eq!(Output::default(), Output::Stdout);
    }

    #[test]
    fn test_output_file() {
        let output = Output::file("/var/log/app.log");
        assert!(output.is_file());
        assert_eq!(output.file_path().unwrap(), Path::new("/var/log/app.log"));
    }

    #[test]
    fn test_output_is_file() {
        assert!(!Output::Stdout.is_file());
        assert!(!Output::Stderr.is_file());
        assert!(Output::File(PathBuf::from("/tmp/test.log")).is_file());
    }

    #[test]
    fn test_output_file_path() {
        assert_eq!(Output::Stdout.file_path(), None);
        assert_eq!(Output::Stderr.file_path(), None);

        let path = PathBuf::from("/tmp/test.log");
        let output = Output::File(path.clone());
        assert_eq!(output.file_path(), Some(path.as_path()));
    }

    #[test]
    fn test_output_display() {
        assert_eq!(Output::Stdout.to_string(), "stdout");
        assert_eq!(Output::Stderr.to_string(), "stderr");
        assert_eq!(
            Output::File(PathBuf::from("/tmp/test.log")).to_string(),
            "file:/tmp/test.log"
        );
    }

    #[test]
    fn test_output_from_str() {
        assert_eq!("stdout".parse::<Output>().unwrap(), Output::Stdout);
        assert_eq!("stderr".parse::<Output>().unwrap(), Output::Stderr);
        assert_eq!("STDOUT".parse::<Output>().unwrap(), Output::Stdout);
        assert_eq!("STDERR".parse::<Output>().unwrap(), Output::Stderr);
    }

    #[test]
    fn test_output_from_str_file() {
        let result = "/tmp/test.log".parse::<Output>().unwrap();
        assert_eq!(result, Output::File(PathBuf::from("/tmp/test.log")));

        let result = "file:/var/log/app.log".parse::<Output>().unwrap();
        assert_eq!(result, Output::File(PathBuf::from("/var/log/app.log")));
    }

    #[test]
    fn test_output_to_stream() {
        let stdout_stream = Stream::from(&Output::Stdout);
        let stderr_stream = Stream::from(&Output::Stderr);
        let file_stream = Stream::from(&Output::File(PathBuf::from("/tmp/test.log")));

        // Can't assert equality on Stream, but we can test the conversions don't panic
        match stdout_stream {
            Stream::Stdout => {}
            _ => panic!("Expected Stream::Stdout"),
        }

        match stderr_stream {
            Stream::Stderr => {}
            _ => panic!("Expected Stream::Stderr"),
        }

        match file_stream {
            Stream::Stdout => {} // Files use Stdout
            _ => panic!("Expected Stream::Stdout for file"),
        }
    }

    #[test]
    fn test_output_eq() {
        assert_eq!(Output::Stdout, Output::Stdout);
        assert_ne!(Output::Stdout, Output::Stderr);
        assert_eq!(
            Output::File(PathBuf::from("/tmp/a.log")),
            Output::File(PathBuf::from("/tmp/a.log"))
        );
        assert_ne!(
            Output::File(PathBuf::from("/tmp/a.log")),
            Output::File(PathBuf::from("/tmp/b.log"))
        );
    }

    #[test]
    fn test_output_clone() {
        let output = Output::File(PathBuf::from("/tmp/test.log"));
        let cloned = output.clone();
        assert_eq!(output, cloned);
    }

    #[test]
    fn test_output_serialize_deserialize() {
        let stdout = Output::Stdout;
        let serialized = serde_json::to_string(&stdout).unwrap();
        assert_eq!(serialized, r#""stdout""#);
        let deserialized: Output = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, Output::Stdout);

        let file = Output::File(PathBuf::from("/tmp/test.log"));
        let serialized = serde_json::to_string(&file).unwrap();
        let deserialized: Output = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, file);
    }

    #[test]
    fn test_output_as_str() {
        assert_eq!(Output::Stdout.as_str(), "stdout");
        assert_eq!(Output::Stderr.as_str(), "stderr");
        assert_eq!(Output::File(PathBuf::from("/tmp/test.log")).as_str(), "file");
    }

    // Test backwards compatibility
    #[test]
    fn test_compat_constants() {
        assert_eq!(STDOUT, "stdout");
        assert_eq!(STDERR, "stderr");
    }

    #[test]
    #[allow(deprecated)]
    fn test_compat_functions() {
        assert_eq!(compat::stdout().unwrap(), "stdout");
        assert_eq!(compat::stderr().unwrap(), "stderr");
    }
}
