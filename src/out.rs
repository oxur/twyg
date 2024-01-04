pub const STDOUT: &str = "stdout";
pub const STDERR: &str = "stderr";

pub fn stdout() -> Option<String> {
    Some(String::from(STDOUT))
}

pub fn stderr() -> Option<String> {
    Some(String::from(STDERR))
}
