use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse, PermanentError};


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
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdCdup {}))
    }
}