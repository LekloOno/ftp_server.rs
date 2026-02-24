use std::fs;
use std::path::PathBuf;

use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse, PermanentError};

#[allow(unused)]
pub struct CmdMkd {
    pathname: String,
}

impl FtpCommand for CmdMkd {
    fn name() -> &'static str where Self: Sized { 
        "MKD" 
    }
}

impl Execute for CmdMkd {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        check_auth!(client);

        match fs::create_dir(&self.pathname) {
            Ok(_) => {
                SuccessResponse::PathnameCreated { 
                    pathname: PathBuf::from(&self.pathname) 
                }.send(client.control())?;
            }
            Err(e) => {
                PermanentError::FileUnavailable {
                    details: format!("{}: Cannot create directory ({})", self.pathname, e)
                }.send(client.control())?;
            }
        }

        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdMkd {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        Ok(Box::new(CmdMkd { pathname: tools::get_one_arg::<Self>(arg)? }))
    }
}