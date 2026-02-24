use std::fs;

use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse, PermanentError};

#[allow(unused)]
pub struct CmdRmd {
    pathname: String,
}

impl FtpCommand for CmdRmd {
    fn name() -> &'static str where Self: Sized { 
        "RMD" 
    }
}

impl Execute for CmdRmd {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        check_auth!(client);

        match fs::remove_dir(&self.pathname) {
            Ok(_) => {
                SuccessResponse::DirectoryRemoved.send(client.control())?;
            }
            Err(e) => {
                PermanentError::FileUnavailable {
                    details: format!("{}: Cannot remove directory ({})", self.pathname, e)
                }.send(client.control())?;
            }
        }

        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdRmd {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        Ok(Box::new(CmdRmd { pathname: tools::get_one_arg::<Self>(arg)? }))
    }
}