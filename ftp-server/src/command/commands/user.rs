use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};

use crate::response::FtpResponse;

#[allow(unused)]
pub struct CmdUser {
    username: String,
}

impl FtpCommand for CmdUser {
    fn name() -> &'static str where Self: Sized {
        "USER"
    }
}

impl Execute for CmdUser {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        client.username = self.username.clone();        
        crate::response::IntermediateReply::UserNameOk.send(client.control())?;
        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdUser {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        Ok(Box::new(CmdUser { username: tools::get_one_arg::<Self>(arg)? }))
    }
}