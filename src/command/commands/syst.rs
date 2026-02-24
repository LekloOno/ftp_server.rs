use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse};

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
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdSyst {}))
    }
}