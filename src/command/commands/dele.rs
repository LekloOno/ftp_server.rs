//! The `DELE` command, which deletes the specified file from the server.

use std::fs;

use ftp_command_parser::command::{BuildableFtpCommand, FtpCommand};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse, PermanentError};

/// The `DELE` command, which deletes the specified file from the server.
/// 
/// # Usage
/// ```rs
/// // Assuming the current directory is "/home/user" and contains a file named "file.txt"
/// client.send_command("DELE file.txt");
/// // The server will attempt to delete the file "/home/user/file.txt"
/// // If successful, the server will respond with "250 File deleted successfully."
/// // If the client is not authenticated, the server will respond with "530 Not logged in."
/// // If the server fails to delete the file (e.g., due to permissions or non-existence), it will respond with "550 Failed to delete file."
/// ```
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
    fn build(arg: Option<String>) -> ftp_command_parser::Result<Box<dyn FtpCommandExec>> {
        Ok(Box::new(CmdDele { pathname: tools::get_one_arg::<Self>(arg)? }))
    }
}