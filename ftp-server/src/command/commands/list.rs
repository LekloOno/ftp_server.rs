use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec}};
use crate::response::{FtpResponse, SuccessResponse, PreliminaryReply, PermanentError};
use std::io::Write;
use std::fs;

/// The `LIST` command, which lists the files and directories in the specified path on the server.
/// If no path is specified, it lists the contents of the current directory.
/// 
/// # Usage
/// ```rs
/// // To list the contents of the current directory
/// client.send_command("LIST");
/// // The server will respond with a list of files and directories in the current directory, followed by "226 Closing data connection. Requested file action successful."
/// // If the client is not authenticated, the server will respond with "530 Not logged in."
/// // If the client fails to establish a data connection (e.g., by not using PASV), the server will respond with "425 Can't open data connection."
/// // If the specified path does not exist or is not a directory, the server will respond with "550 Failed to list directory."
/// ```
#[allow(unused)]
pub struct CmdList {
    pathname: Option<String>,
}

impl FtpCommand for CmdList {
    fn name() -> &'static str where Self: Sized {
        "LIST"
    }
}

impl Execute for CmdList {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        check_auth!(client);

        let (ctrl_stream, data_stream) = client.streams();

        let Some(data_stream) = data_stream else {
            PermanentError::Generic {
                    details: "Use PASV first.".to_string()
            }.send(client.control())?;
            return Ok(());
        };

        PreliminaryReply::AboutToOpenDataConnection.send(ctrl_stream)?;

        let target_dir = self.pathname.clone().unwrap_or_else(|| ".".to_string());
        
        if let Ok(entries) = fs::read_dir(target_dir) {
            for entry in entries.flatten() {
                let metadata = entry.metadata()?;
                let name = entry.file_name().into_string().unwrap_or_default();
                let dir_flag = if metadata.is_dir() { "d" } else { "-" };
                let size = metadata.len();
                let line = format!("{}rw-r--r-- 1 ftp ftp {} Jan 01 00:00 {}\r\n", dir_flag, size, name);
                let _ = data_stream.write_all(line.as_bytes());
            }
        }
        
        client.drop_data();
        SuccessResponse::ClosingDataConnection.send(client.control())?;

        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdList {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        let Some(arg_str) = &arg else {
            return Ok(Box::new(CmdList {pathname: None}));
        };

        let args = arg_str
            .split_whitespace()
            .collect::<Vec<&str>>();

        let pathname = match args.len() {
            0 => None,
            1 => Some(args[0].to_string()),
            _ => Err(FtpParseError::UnvalidCmdArg { arg, code: Self::name().to_string() })?,
        };

        Ok(Box::new(CmdList {pathname}))
    }
}