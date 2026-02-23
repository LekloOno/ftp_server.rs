use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};

use std::io::Write;
use crate::response::{FtpResponse, SuccessResponse};

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
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        client.authentified = true;
        SuccessResponse::UserLoggedIn.send(client.control())?;
        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdPass {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        Ok(Box::new(CmdPass { password: tools::get_one_arg::<Self>(arg)? }))
    }
}