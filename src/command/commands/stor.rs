//! The `STOR` command, which stores a file sent by the client on the server at the specified pathname.

use std::fs::OpenOptions;
use std::io::{copy, Seek, SeekFrom};

use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse, PreliminaryReply, PermanentError};

/// The `STOR` command, which stores a file sent by the client on the server at the specified pathname.
/// 
/// # Usage
/// ```rs
/// // Assuming the current directory is "/home/user" and the client wants to upload a file named "upload.txt"
/// client.send_command("STOR upload.txt");
/// // The server will attempt to create the file "/home/user/upload.txt" and write the contents sent by the client over the data connection into it.
/// // If successful, the server will respond with "150 File status okay; about to open data connection." and then receive the file contents over the data connection, followed by "226 Closing data connection. Requested file action successful."
/// // If the client is not authenticated, the server will respond with "530 Not logged in."
/// // If the server fails to create the file (e.g., due to permissions), it will respond with "550 Failed to create file."
/// // If the client has not established a data connection (e.g., by using PASV), the server will respond with "425 Can't open data connection."
/// ```
#[allow(unused)]
pub struct CmdStor {
    pathname: String,
}

impl FtpCommand for CmdStor {
    fn name() -> &'static str where Self: Sized { 
        "STOR" 
    }
}

impl Execute for CmdStor {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        check_auth!(client);

        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(client.restart_offset == 0)
            .open(&self.pathname) 
        {
            Ok(f) => f,
            Err(_) => {
                PermanentError::FileUnavailable {
                    details: format!("Cannot create file {}", self.pathname)
                }.send(client.control())?;
                return Ok(());
            }
        };

        if client.restart_offset > 0 {
            if let Err(e) = file.seek(SeekFrom::Start(client.restart_offset)) {
                PermanentError::FileUnavailable {
                    details: format!("Cannot seek file: {}", e)
                }.send(client.control())?;
                return Ok(());
            }
            client.restart_offset = 0;
        }

        let (ctrl_stream, data_stream) = client.streams();
        let Some(mut data_stream) = data_stream else {
            PermanentError::Generic {
                details: "Use PASV first.".to_string()
            }.send(ctrl_stream)?;
            return Ok(());
        };

        PreliminaryReply::AboutToOpenDataConnection.send(ctrl_stream)?;

        let _ = copy(&mut data_stream, &mut file);

        client.drop_data();

        SuccessResponse::ClosingDataConnection.send(client.control())?;

        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdStor {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        Ok(Box::new(CmdStor { pathname: tools::get_path_arg::<Self>(arg)? }))
    }
}