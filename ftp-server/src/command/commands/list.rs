use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec}};

use std::io::Write;

#[allow(unused)]
pub struct CmdList {
    pathname: Option<String>,
}

impl FtpCommand for CmdList {
    fn name() -> &'static str where Self: Sized {
        "LIST"
    }
}

impl Execute for CmdList {
    fn execute(&self, client: &mut FtpClient) {
        let _ = client.control().write_all(b"200 Command recognized\r\n");
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdList {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        let Some(arg_str) = &arg else {
            return Ok(Box::new(CmdList {pathname: None}));
        };

        let args = arg_str
            .split_whitespace()
            .collect::<Vec<&str>>();

        let pathname = match args.len() {
            0 => None,
            1 => Some(args[0].to_string()),
            _ => Err(FtpParseError::UnvalidCmdArg { arg, code: Self::name().to_string() })?,
        };

        Ok(Box::new(CmdList {pathname}))
    }
}