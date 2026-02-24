use std::fs;

use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse, PermanentError};

#[allow(unused)]
pub struct CmdDele {
    pathname: String,
}

impl FtpCommand for CmdDele {
    fn name() -> &'static str where Self: Sized { 
        "DELE" 
    }
}

impl Execute for CmdDele {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        check_auth!(client);

        match fs::remove_file(&self.pathname) {
            Ok(_) => {
                SuccessResponse::FileDeleted.send(client.control())?;
            }
            Err(e) => {
                PermanentError::FileUnavailable {
                    details: format!("{}: Cannot delete file ({})", self.pathname, e)
                }.send(client.control())?;
            }
        }

        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdDele {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        Ok(Box::new(CmdDele { pathname: tools::get_one_arg::<Self>(arg)? }))
    }
}