use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};

use crate::response::{FtpResponse, SuccessResponse, PermanentError};

/// The `CWD` command, which changes the current working directory on the server to the specified path.
/// 
/// # Usage
/// ```rs
/// // Assuming the current directory is "/home/user"
/// client.send_command("CWD /home/user/documents");
/// // The server will attempt to change the current directory to "/home/user/documents"
/// // If successful, the server will respond with "250 Requested file action okay, completed."
/// // If the client is not authenticated, the server will respond with "530 Not logged in."
/// // If the server fails to change the directory (e.g., due to permissions or non-existence), it will respond with "550 Failed to change directory."
/// ```
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

        if let Err(err) = client.server().set_current_dir(&self.pathname) {
            match err {
                crate::Error::PermissionDenied =>
                    PermanentError::FileUnavailable {
                        details: "Permission denied".to_string()
                    },
                _ => 
                    PermanentError::Generic {
                        details: "Failed to change directory".to_string()
                    },
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