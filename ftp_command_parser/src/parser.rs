use std::collections::HashMap;

use thiserror::Error;

use crate::command::{BuildableFtpCommand, FtpCommand};

pub struct FtpCommandParser<T: FtpCommand + ?Sized + 'static> {
    commands: HashMap<&'static str, FtpCommandBuilder<T>>,
}

pub struct FtpCommandBuilder<T: FtpCommand + ?Sized + 'static> {
    constructor: Box<dyn Fn(Option<String>) -> Result<Box<T>, FtpParseError> + Send + Sync>,
}

impl<T: FtpCommand + ?Sized> FtpCommandBuilder<T> {
    pub fn build(&self, arg: Option<String>) -> Result<Box<T>, FtpParseError> {
        (*self.constructor)(arg)
    }
}

impl<T: FtpCommand + ?Sized> FtpCommandParser<T> {
    pub fn new() -> Self {
        Self { commands: HashMap::new() }
    }

    pub fn register<B: BuildableFtpCommand<T> + 'static>(&mut self) {
        self.commands.insert(
            B::name(),
            FtpCommandBuilder::<T>{ constructor: Box::new(B::build)}
        );
    }

    pub fn get(&self, code: &'static str) -> Result<&FtpCommandBuilder<T>, FtpParseError> {
        self.commands
            .get(code)
            .ok_or_else(|| FtpParseError::CmdNameNotFound { code })
    }
}

#[derive(Error, Debug)]
pub enum FtpParseError {
    #[error("'{code}' is not a registered command code")]
    CmdNameNotFound {code: &'static str},
    #[error("'{}' argument is invalid for command code '{code}'",
        match arg {
            Some(content) => content,
            None => "empty",
        })]
    UnvalidCmdArg {arg: Option<String>, code: &'static str},
}