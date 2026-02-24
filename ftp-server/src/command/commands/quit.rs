use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};

use std::io::Write;

/// The `QUIT` command, which terminates the FTP session between the client and server.
/// 
/// # Usage
/// ```rs
/// // To terminate the FTP session
/// client.send_command("QUIT");
/// // The server will respond with "221 Goodbye" and close the connection if successful.
/// // If the client is not authenticated, the server will still respond with "221 Goodbye" and close the connection, as the QUIT command is valid regardless of authentication status.
/// ```
#[allow(unused)]
pub struct CmdQuit {}

impl FtpCommand for CmdQuit {
    fn name() -> &'static str where Self: Sized {
        "QUIT"
    }
}

impl Execute for CmdQuit {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        let _ = client.control().write_all(b"221 Goodbye\r\n");
        Err(crate::Error::Quit)
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdQuit {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdQuit {}))
    }
}