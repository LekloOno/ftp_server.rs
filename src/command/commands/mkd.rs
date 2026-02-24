//! The `MKD` command, which creates a new directory on the server with the specified pathname.

use std::fs;
use std::path::PathBuf;

use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse, PermanentError};

/// The `MKD` command, which creates a new directory on the server with the specified pathname.
/// 
/// # Usage
/// ```rs
/// // To create a new directory named "new_folder" in the current directory
/// client.send_command("MKD new_folder");
/// // The server will attempt to create the directory "/current/directory/new_folder"
/// // If successful, the server will respond with "257 "/current/directory/new_folder" Directory created."
/// // If the client is not authenticated, the server will respond with "530 Not logged in."
/// // If the server fails to create the directory (e.g., due to permissions or if the directory already exists), it will respond with "550 Failed to create directory."
/// ```
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