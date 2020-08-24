/// System URI error variants.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    /// FFI String conversion error.
    #[error(transparent)]
    IntoStringError(#[from] std::ffi::IntoStringError),
    /// Utf-8 error.
    #[error("Utf-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[cfg(target_os = "linux")]
    /// XDG error.
    #[error("Executing `xdg-open {0}` failed: {1}")]
    XdgOpenError(String, String),
    #[cfg(target_os = "windows")]
    /// Open error.
    #[error("Using ShellExecuteW to open URL failed with code {0}")]
    ShellOpenError(i32),
    /// Other errors.
    #[error(transparent)]
    Other(#[from] anyhow::Error)
}

impl From<&'static str> for Error {
    fn from(v: &'static str) -> Self {
        anyhow::anyhow!(v).into()
    }
}
