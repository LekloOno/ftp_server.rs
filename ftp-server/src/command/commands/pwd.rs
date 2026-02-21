use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::command::{Execute, FtpCommandExec, tools};

use std::net::TcpStream;
use std::io::Write;

#[allow(unused)]
pub struct CmdPwd {}

impl FtpCommand for CmdPwd {
    fn name() -> &'static str where Self: Sized {
        "PWD"
    }
}

impl Execute for CmdPwd {
    fn execute(&self, stream: &mut TcpStream) {
        let _ = stream.write_all(b"257 \"/\" is the current directory\r\n");
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdPwd {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdPwd {}))
    }
}