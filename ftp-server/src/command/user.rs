use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::command::{Execute, FtpCommandExec};

#[allow(unused)]
pub struct CmdUser {
    username: String,
}

impl FtpCommand for CmdUser {
    fn name() -> &'static str where Self: Sized {
        "USER"
    }
}

impl Execute for CmdUser {}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdUser {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        // A username must be provided
        let Some(username) = &arg else {
            return Err(FtpParseError::UnvalidCmdArg { arg, code: Self::name() });
        };

        // We could be smarter and iterate then stop once we have more than one args
        // This would avoid potential expensive collect .. Probably negligeable anyways
        let args = username.split_whitespace().collect::<Vec<&str>>();

        // Exactly one argument must be provided !
        if args.len() != 1 {
            return Err(FtpParseError::UnvalidCmdArg { arg, code: Self::name() });
        }

        Ok(Box::new(CmdUser { username: args[0].to_string() }))
    }
}