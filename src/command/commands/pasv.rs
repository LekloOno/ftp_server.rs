use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse};
use std::net::TcpListener;


/// The `PASV` command, which puts the server into passive mode for data transfer.
/// 
/// # Usage
/// ```rs
/// // To enter passive mode for data transfer
/// client.send_command("PASV");
/// // The server will respond with "227 Entering Passive Mode (h1,h2,h3,h4,p1,p2)" where h1-h4 represent the server's IP address and p1-p2 represent the port number for the data connection.
/// // If the client is not authenticated, the server will respond with "530 Not logged in."
/// // After receiving the PASV response, the client should establish a data connection to the specified IP and port for subsequent data transfer commands (e.g., RETR, STOR).
/// ```
pub struct CmdPasv {}

impl FtpCommand for CmdPasv {
    fn name() -> &'static str where Self: Sized {
        "PASV"
    }
}

impl Execute for CmdPasv {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        check_auth!(client);

        let local_addr = client.control().local_addr()?;
        let ip = match local_addr.ip() {
            std::net::IpAddr::V4(ipv4) => ipv4.octets(),
            _ => [127, 0, 0, 1],
        };

        let listener = TcpListener::bind(format!("{}:0", local_addr.ip()))?;
        let port = listener.local_addr()?.port();
        
        let p1 = (port / 256) as u8;
        let p2 = (port % 256) as u8;

        SuccessResponse::EnteringPasv {
            h1: ip[0], h2: ip[1], h3: ip[2], h4: ip[3],
            p1, p2
        }.send(client.control())?;

        let (data_stream, _) = listener.accept()?;
        
        client.set_data(data_stream);

        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdPasv {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        tools::get_no_args::<Self>(arg)?;
        Ok(Box::new(CmdPasv {}))
    }
}