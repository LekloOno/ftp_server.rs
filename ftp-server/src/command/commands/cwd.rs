use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};

use crate::response::{FtpResponse, SuccessResponse, PermanentError};
use std::env;

#[allow(unused)]
pub struct CmdCwd {
    pathname: String,
}

impl FtpCommand for CmdCwd {
    fn name() -> &'static str where Self: Sized {
        "CWD"
    }
}

impl Execute for CmdCwd {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        crate::check_auth!(client);

        if let Err(_) = env::set_current_dir(&self.pathname) {
            PermanentError::Generic { 
                details: "Failed to change directory".to_string() 
            }.send(client.control())?;
        } 
        else {
            SuccessResponse::DirectoryChanged.send(client.control())?;
        }
        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdCwd {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        Ok(Box::new(CmdCwd { pathname: tools::get_one_arg::<Self>(arg)? }))
    }
}