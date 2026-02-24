//! Defines the parser structure and boilerplate.

use std::collections::HashMap;
use crate::command::{BuildableFtpCommand, FtpCommand};

/// A command parser used to build new commands from received
/// command codes, such as PWD, CWD, etc.
pub struct FtpCommandParser<T: FtpCommand + ?Sized + 'static> {
    commands: HashMap<&'static str, FtpCommandBuilder<T>>,
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

    pub fn get(&self, code: &str) -> crate::Result<&FtpCommandBuilder<T>> {
        self.commands
            .get(code)
            .ok_or_else(|| crate::Error::CmdNameNotFound { code: code.to_string() })
    }
}

/// Wraps a constructor of a given command. Intended to be used within the [`FtpCommandParser`].
/// 
/// This could be a type alias instead, but defining a struct allows for some more explicitness
/// in the final usage, notably through the explicit call of [`FtpCommandBuilder::build`].
pub struct FtpCommandBuilder<T: FtpCommand + ?Sized + 'static> {
    constructor: Box<dyn Fn(Option<String>) -> crate::Result<Box<T>> + Send + Sync>,
}

impl<T: FtpCommand + ?Sized> FtpCommandBuilder<T> {
    pub fn build(&self, arg: Option<String>) -> crate::Result<Box<T>> {
        (*self.constructor)(arg)
    }
}
