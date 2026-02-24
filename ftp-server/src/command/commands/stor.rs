use std::fs::File;
use std::io::copy;

use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse, PreliminaryReply, PermanentError};

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

        let mut file = match File::create(&self.pathname) {
            Ok(f) => f,
            Err(_) => {
                PermanentError::FileUnavailable {
                    details: format!("Cannot create file {}", self.pathname)
                }.send(client.control())?;
                return Ok(());
            }
        };

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
        Ok(Box::new(CmdStor { pathname: tools::get_one_arg::<Self>(arg)? }))
    }
}