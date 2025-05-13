use std::process::ExitStatus;

/// Checks if the exit status is for a file that wasn't changed (SOPS returns 200)
pub fn is_file_unchanged_status(status: &ExitStatus) -> bool {
    if let Some(code) = status.code() {
        return code == 200;
    }
    false
}