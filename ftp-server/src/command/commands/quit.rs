use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};

use std::io::Write;

#[allow(unused)]
pub struct CmdQuit {}

impl FtpCommand for CmdQuit {
    fn name() -> &'static str where Self: Sized {
        "QUIT"
    }
}

impl Execute for CmdQuit {
    fn execute(&self, client: &mut FtpClient) {
        let _ = client.control().write_all(b"200 Command recognized\r\n");
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdQuit {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdQuit {}))
    }
}