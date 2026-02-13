use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::command::{Execute, FtpCommandExec, tools};

#[allow(unused)]
pub struct CmdPwd {}

impl FtpCommand for CmdPwd {
    fn name() -> &'static str where Self: Sized {
        "PWD"
    }
}

impl Execute for CmdPwd {}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdPwd {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdPwd {}))
    }
}