use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::command::{Execute, FtpCommandExec};

pub struct CmdPasv {}

impl FtpCommand for CmdPasv {
    fn name() -> &'static str where Self: Sized {
        "PASV"
    }
}

impl Execute for CmdPasv {}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdPasv {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        // No arguments are expected for PASV commands 
        if let Some(_) = &arg {
            return Err(FtpParseError::UnvalidCmdArg { arg, code: Self::name() });
        };

        Ok(Box::new(CmdPasv {}))
    }
}