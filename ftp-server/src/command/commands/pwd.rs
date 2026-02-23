use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec, tools}, response::{FtpResponse, SuccessResponse}};

#[allow(unused)]
pub struct CmdPwd {}

impl FtpCommand for CmdPwd {
    fn name() -> &'static str where Self: Sized {
        "PWD"
    }
}

impl Execute for CmdPwd {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        check_auth!(client);

        SuccessResponse::PathnameCreated { pathname: client.server().current_dir()? }
            .send(client.control())
            .map_err(crate::Error::from)
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdPwd {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdPwd {}))
    }
}