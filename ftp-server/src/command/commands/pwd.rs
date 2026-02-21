use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};

use std::io::Write;

#[allow(unused)]
pub struct CmdPwd {}

impl FtpCommand for CmdPwd {
    fn name() -> &'static str where Self: Sized {
        "PWD"
    }
}

impl Execute for CmdPwd {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        let _ = client.control().write_all(b"200 Command recognized\r\n");
        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdPwd {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdPwd {}))
    }
}