//! The `REST` command, which allows the client to specify a byte offset for resuming file transfers.

use ftp_command_parser::command::{BuildableFtpCommand, FtpCommand};
use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, IntermediateReply};

/// The `REST` command, which allows the client to specify a byte offset for resuming file transfers.
/// 
/// # Usage
/// ```rs
/// // To specify a byte offset for resuming a file transfer:
/// client.send_command("REST 1024");
/// // This indicates that the client wants to resume the transfer from byte offset 1024.
/// // The server will respond with "350 Restarting at 1024. Send STORE or RETRIEVE to initiate transfer" if the offset is accepted.
/// // If the client provides an invalid offset (e.g., non-numeric), the server will respond with "501 Syntax error in parameters or arguments."
/// ```
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
    fn build(arg: Option<String>) -> ftp_command_parser::Result<Box<dyn FtpCommandExec>> {
        let arg_str = tools::get_one_arg::<Self>(arg.clone())?;
        
        let offset = arg_str.parse::<u64>().map_err(|_| ftp_command_parser::Error::UnvalidCmdArg { arg, code: Self::name().to_string() })?;
        
        Ok(Box::new(CmdRest { offset }))
    }
}