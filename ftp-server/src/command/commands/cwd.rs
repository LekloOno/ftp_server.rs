use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};

use std::io::Write;

#[allow(unused)]
pub struct CmdCwd {
    pathname: String,
}

impl FtpCommand for CmdCwd {
    fn name() -> &'static str where Self: Sized {
        "CWD"
    }
}

impl Execute for CmdCwd {
    fn execute(&self, client: &mut FtpClient) {
        let _ = client.control().write_all(b"200 Command recognized\r\n");
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdCwd {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        Ok(Box::new(CmdCwd { pathname: tools::get_one_arg::<Self>(arg)? }))
    }
}