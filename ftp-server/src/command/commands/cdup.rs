use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::command::{Execute, FtpCommandExec, tools};

#[allow(unused)]
pub struct CmdCdup {}

impl FtpCommand for CmdCdup {
    fn name() -> &'static str where Self: Sized {
        "CDUP"
    }
}

impl Execute for CmdCdup {}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdCdup {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdCdup {}))
    }
}