//! The `RMD` command, which removes the specified directory from the server.

use std::fs;

use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse, PermanentError};

/// The `RMD` command, which removes the specified directory from the server.
/// 
/// # Usage
/// ```rs
/// // Assuming the current directory is "/home/user" and contains a directory named "old_folder"
/// client.send_command("RMD old_folder");
/// // The server will attempt to remove the directory "/home/user/old_folder"
/// // If successful, the server will respond with "250 Directory removed successfully."
/// // If the client is not authenticated, the server will respond with "530 Not logged in."
/// // If the server fails to remove the directory (e.g., due to permissions, non-existence, or if the directory is not empty), it will respond with "550 Failed to remove directory."
/// ```
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