use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::command::{Execute, FtpCommandExec, tools};

use std::net::TcpStream;
use std::io::Write;

#[allow(unused)]
pub struct CmdUser {
    username: String,
}

impl FtpCommand for CmdUser {
    fn name() -> &'static str where Self: Sized {
        "USER"
    }
}

impl Execute for CmdUser {
    fn execute(&self, stream: &mut TcpStream) {
        let _ = stream.write_all(b"331 Password required\r\n");
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdUser {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        Ok(Box::new(CmdUser { username: tools::get_one_arg::<Self>(arg)? }))
    }
}