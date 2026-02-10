use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::command::{Execute, FtpCommandExec};

#[allow(unused)]
pub struct CmdPass {
    password: String,
}

impl FtpCommand for CmdPass {
    fn name() -> &'static str where Self: Sized {
        "PASS"
    }
}

impl Execute for CmdPass {}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdPass {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        // A username must be provided
        let Some(password) = &arg else {
            return Err(FtpParseError::UnvalidCmdArg { arg, code: Self::name() });
        };

        // We could be smarter and iterate then stop once we have more than one args
        // This would avoid potential expensive collect .. Probably negligeable anyways
        let args = password.split_whitespace().collect::<Vec<&str>>();

        // Exactly one argument must be provided !
        if args.len() != 1 {
            return Err(FtpParseError::UnvalidCmdArg { arg, code: Self::name() });
        }

        Ok(Box::new(CmdPass { password: args[0].to_string() }))
    }
}