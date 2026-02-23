use ftp_command_parser::command::{FtpCommand, FtpCommandDyn};

use crate::client::FtpClient;

pub(self) mod tools;

pub mod commands;

/// # Notes
/// We define Execute as an independant trait instead of binding it to FtpCommand as Execute: FtpCommand.
/// Execute shouldn't be bound to the parsing method. They are not linked.
/// 
/// We instead use a blanket implementation [`FtpCommandExec`] that binds them.
/// The trait is empty for now, we focus on implementing Parse.
pub trait Execute {
    fn execute<'a>(&self, client: &mut FtpClient<'a>) -> crate::Result<()>;
}

pub trait FtpCommandExec : FtpCommand + FtpCommandDyn + Execute {}

impl<T> FtpCommandExec for T where T: FtpCommand + FtpCommandDyn + Execute {}