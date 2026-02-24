use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, IntermediateReply};

#[allow(unused)]
pub struct CmdRest {
    offset: u64,
}

impl FtpCommand for CmdRest {
    fn name() -> &'static str where Self: Sized { 
        "REST" 
    }
}

impl Execute for CmdRest {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        client.restart_offset = self.offset;
        IntermediateReply::RestartMarker { offset: self.offset }.send(client.control())?;
        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdRest {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        let arg_str = tools::get_one_arg::<Self>(arg.clone())?;
        
        let offset = arg_str.parse::<u64>().map_err(|_| FtpParseError::UnvalidCmdArg { arg, code: Self::name().to_string() })?;
        
        Ok(Box::new(CmdRest { offset }))
    }
}