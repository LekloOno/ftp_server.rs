use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::command::{Execute, FtpCommandExec, tools};

use std::net::TcpStream;
use std::io::Write;

#[allow(unused)]
pub struct CmdPass {
    password: String,
}

impl FtpCommand for CmdPass {
    fn name() -> &'static str where Self: Sized {
        "PASS"
    }
}

impl Execute for CmdPass {
    fn execute(&self, stream: &mut TcpStream) {
        let _ = stream.write_all(b"230 User logged in\r\n");
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdPass {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        Ok(Box::new(CmdPass { password: tools::get_one_arg::<Self>(arg)? }))
    }
}