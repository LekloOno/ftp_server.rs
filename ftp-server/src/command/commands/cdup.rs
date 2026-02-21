use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};

use std::io::Write;

#[allow(unused)]
pub struct CmdCdup {}

impl FtpCommand for CmdCdup {
    fn name() -> &'static str where Self: Sized {
        "CDUP"
    }
}

impl Execute for CmdCdup {
    fn execute(&self, client: &mut FtpClient) {
        let _ = client.control().write_all(b"200 Command recognized\r\n");
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdCdup {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdCdup {}))
    }
}