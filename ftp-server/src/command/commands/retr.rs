use std::fs::File;
use std::io::copy;

use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse, PreliminaryReply, PermanentError};

/// The `RETR` command, which retrieves a file from the server and sends it to the client over a data connection.
/// 
/// # Usage
/// ```rs
/// // Assuming the current directory is "/home/user" and contains a file named "file.txt"
/// client.send_command("RETR file.txt");
/// // The server will attempt to retrieve the file "/home/user/file.txt"
/// // If successful, the server will respond with "150 File status okay; about to open data connection." and then send the file contents over the data connection, followed by "226 Closing data connection. Requested file action successful."
/// // If the client is not authenticated, the server will respond with "530 Not logged in."
/// // If the server fails to retrieve the file (e.g., due to permissions or non-existence), it will respond with "550 Failed to retrieve file."
/// // If the client has not established a data connection (e.g., by using PASV), the server will respond with "425 Can't open data connection."
/// ```
#[allow(unused)]
pub struct CmdRetr {
    pathname: String,
}

impl FtpCommand for CmdRetr {
    fn name() -> &'static str where Self: Sized { 
        "RETR" 
    }
}

impl Execute for CmdRetr {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        check_auth!(client);

        let mut file = match File::open(&self.pathname) {
            Ok(f) => f,
            Err(_) => {
                PermanentError::FileUnavailable {
                    details: format!("{}: No such file or permission denied", self.pathname)
                }.send(client.control())?;
                return Ok(());
            }
        };

        let (ctrl_stream, data_stream) = client.streams();
        let Some(data_stream) = data_stream else {
            PermanentError::Generic {
                details: "Use PASV first.".to_string()
            }.send(ctrl_stream)?;
            return Ok(());
        };

        PreliminaryReply::AboutToOpenDataConnection.send(ctrl_stream)?;

        let _ = copy(&mut file, data_stream);

        client.drop_data();

        SuccessResponse::ClosingDataConnection.send(client.control())?;

        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdRetr {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        Ok(Box::new(CmdRetr { pathname: tools::get_one_arg_relaxed::<Self>(arg)? }))
    }
}