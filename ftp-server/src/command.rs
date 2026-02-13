use ftp_command_parser::command::FtpCommand;

pub(self) mod tools;

pub mod user;
pub mod pass;
pub mod pasv;
pub mod cwd;
pub mod pwd;

/// # Notes
/// We define Execute as an independant trait instead of binding it to FtpCommand as Execute: FtpCommand.
/// Execute shouldn't be bound to the parsing method. They are not linked.
/// 
/// We instead use a blanket implementation FtpCommandExec that binds them.
/// The trait is empty for now, we focus on implementing Parse.
pub trait Execute {}

pub trait FtpCommandExec : FtpCommand + Execute {}

impl<T> FtpCommandExec for T where T: FtpCommand + Execute {}