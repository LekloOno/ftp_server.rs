use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}, response::PermanentError};

use crate::response::{FtpResponse, SuccessResponse};

#[allow(unused)]
pub struct CmdPass {
    password: String,
}

impl FtpCommand for CmdPass {
    fn name() -> &'static str where Self: Sized {
        "PASS"
    }
}

impl Execute for CmdPass {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        let Some(pswd_opt) = client.server().users.get(&client.username) else {
            PermanentError::NotLoggedIn { details: "User not found".to_string() }
                .send(client.control())?;
            return Ok(());
        };

        if let Some(pswd) = pswd_opt && pswd != &self.password {
            PermanentError::NotLoggedIn { details: "Incorrect Password".to_string() }
                .send(client.control())?;
            return Ok(());
        }

        client.authentified = true;
        SuccessResponse::UserLoggedIn.send(client.control())?;
        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdPass {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        Ok(Box::new(CmdPass { password: tools::get_one_arg::<Self>(arg)? }))
    }
}