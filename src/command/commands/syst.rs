//! The `SYST` command, which returns the type of operating system the server is running on.

use ftp_command_parser::command::{BuildableFtpCommand, FtpCommand};
use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse};

/// The `SYST` command, which returns the type of operating system the server is running on.
/// 
/// # Usage
/// ```rs
/// client.send_command("SYST");
/// // The server will respond with "215 UNIX Type: L8" if it is running a UNIX-like operating system.
/// // If the server is running a different type of operating system, it will respond with a different message indicating the system type.
/// ```
#[allow(unused)]
pub struct CmdSyst {}

impl FtpCommand for CmdSyst {
    fn name() -> &'static str where Self: Sized { 
        "SYST" 
    }
}

impl Execute for CmdSyst {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        SuccessResponse::SystemType.send(client.control())?;
        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdSyst {
    fn build(arg: Option<String>) -> ftp_command_parser::Result<Box<dyn FtpCommandExec>> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdSyst {}))
    }
}