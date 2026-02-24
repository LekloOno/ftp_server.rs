use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("'{code}' is not a registered command code")]
    CmdNameNotFound {code: String},
    #[error("'{}' argument is invalid for command code '{code}'",
        match arg {
            Some(content) => content,
            None => "empty",
        })]
    UnvalidCmdArg {arg: Option<String>, code: String},
}