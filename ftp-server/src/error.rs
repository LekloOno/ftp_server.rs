//! Defines the server-side possible errors.
//! 
//! There are very few as most possible errors are handled directly within
//! the commands execution, in a safe and expected way, typically by sending
//! back [`crate::response::FtpResponse`] through the control stream.
 
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Connection closed")]
    Quit,
}