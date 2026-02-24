//! The `CDUP` command, which changes the current directory to the parent directory.

use ftp_command_parser::command::{BuildableFtpCommand, FtpCommand};
use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse, PermanentError};

/// The `CDUP` command, which changes the current directory to the parent directory.
///
/// # Usage
/// ```rs
/// // Assuming the current directory is "/home/user"
/// client.send_command("CDUP");
/// // The server will attempt to change the current directory to "/home"
/// // If successful, the server will respond with "250 Requested file action okay, completed."
/// // If the client is not authenticated, the server will respond with "530 Not logged in."
/// // If the server fails to change the directory (e.g., due to permissions), it will respond with "550 Failed to change directory to parent."
/// ```
#[allow(unused)]
pub struct CmdCdup {}

impl FtpCommand for CmdCdup {
    fn name() -> &'static str where Self: Sized {
        "CDUP"
    }
}

impl Execute for CmdCdup {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        crate::check_auth!(client);

        if let Err(err) = client.server().set_current_dir("..") {
            match err {
                crate::Error::PermissionDenied =>
                    PermanentError::FileUnavailable {
                        details: "Permission denied".to_string()
                    },
                _ => 
                    PermanentError::Generic {
                        details: "Failed to change directory to parent".to_string()
                    },
            }.send(client.control())?;
        } else {
            SuccessResponse::DirectoryChanged.send(client.control())?;
        }
        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdCdup {
    fn build(arg: Option<String>) -> ftp_command_parser::Result<Box<dyn FtpCommandExec>> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdCdup {}))
    }
}