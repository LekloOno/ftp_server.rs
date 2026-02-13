use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::command::{Execute, FtpCommandExec, tools};

pub struct CmdPasv {}

impl FtpCommand for CmdPasv {
    fn name() -> &'static str where Self: Sized {
        "PASV"
    }
}

impl Execute for CmdPasv {}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdPasv {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdPasv {}))
    }
}