use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};

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
    fn execute(&self, client: &mut FtpClient) {
        let _ = client.control().write_all(b"200 Command recognized\r\n");
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdPass {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        Ok(Box::new(CmdPass { password: tools::get_one_arg::<Self>(arg)? }))
    }
}