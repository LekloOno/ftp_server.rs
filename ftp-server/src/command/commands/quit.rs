use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::command::{Execute, FtpCommandExec, tools};

use std::net::TcpStream;
use std::io::Write;

#[allow(unused)]
pub struct CmdQuit {}

impl FtpCommand for CmdQuit {
    fn name() -> &'static str where Self: Sized {
        "QUIT"
    }
}

impl Execute for CmdQuit {
    fn execute(&self, stream: &mut TcpStream) {
        let _ = stream.write_all(b"221 Goodbye\r\n");
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdQuit {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdQuit {}))
    }
}