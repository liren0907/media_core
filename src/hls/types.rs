use std::error::Error;
use std::fmt;

/// Errors that can occur during HLS VOD conversion
#[derive(Debug)]
pub enum HLSError {
    /// I/O related errors (file not found, permission denied, etc.)
    IoError(String),
    /// FFmpeg process errors
    FFmpegError(String),
    /// Invalid input parameters
    InvalidInput(String),
}

impl fmt::Display for HLSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HLSError::IoError(msg) => write!(f, "I/O Error: {}", msg),
            HLSError::FFmpegError(msg) => write!(f, "FFmpeg Error: {}", msg),
            HLSError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
        }
    }
}

impl Error for HLSError {}

impl From<std::io::Error> for HLSError {
    fn from(err: std::io::Error) -> Self {
        HLSError::IoError(err.to_string())
    }
}
