use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}, response::PermanentError};

use crate::response::FtpResponse;

/// The `USER` command, which initiates the login process by providing the username of the user attempting to log in.
/// 
/// # Usage
/// ```rs
/// // To log in as a user named "test_user"
/// client.send_command("USER test_user");
/// // The server will check if the user "test_user" exists
/// // If the user exists, the server will respond with "331 User name okay, need password."
/// // If the user does not exist, the server will respond with "530 Not logged in."
/// ```
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
        if client.server().users.get(&self.username).is_none() {
            PermanentError::NotLoggedIn { details: "User not found".to_string() }
                .send(client.control())?;
            return Ok(());
        };
        
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