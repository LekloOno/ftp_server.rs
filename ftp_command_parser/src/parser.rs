use std::collections::HashMap;
use crate::command::{BuildableFtpCommand, FtpCommand};

pub struct FtpCommandParser<T: FtpCommand + ?Sized + 'static> {
    commands: HashMap<&'static str, FtpCommandBuilder<T>>,
}

pub struct FtpCommandBuilder<T: FtpCommand + ?Sized + 'static> {
    constructor: Box<dyn Fn(Option<String>) -> crate::Result<Box<T>> + Send + Sync>,
}

impl<T: FtpCommand + ?Sized> FtpCommandBuilder<T> {
    pub fn build(&self, arg: Option<String>) -> crate::Result<Box<T>> {
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

    pub fn get(&self, code: &str) -> crate::Result<&FtpCommandBuilder<T>> {
        self.commands
            .get(code)
            .ok_or_else(|| crate::Error::CmdNameNotFound { code: code.to_string() })
    }
}