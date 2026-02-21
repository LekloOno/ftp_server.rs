use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Connection closed")]
    Quit,
}