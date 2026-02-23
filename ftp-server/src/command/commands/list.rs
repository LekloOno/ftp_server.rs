use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec}};
use crate::response::{FtpResponse, SuccessResponse, PreliminaryReply, PermanentError};
use std::io::Write;
use std::fs;

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

        let mut data_stream = match client.data.take() {
            Some(stream) => stream,
            None => {
                PermanentError::Generic {
                    details: "Use PASV first.".to_string()
                }.send(client.control())?;
                return Ok(());
            }
        };

        PreliminaryReply::AboutToOpenDataConnection.send(client.control())?;

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

        drop(data_stream);

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