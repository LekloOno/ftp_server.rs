//! The `PORT` command, which allows the client to specify an IP address and port for the server to connect to for data transfer.

use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use ftp_command_parser::command::{BuildableFtpCommand, FtpCommand};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse, PermanentError};

/// Implements the PORT command, which allows the client to specify an IP address and port for the server to connect to for data transfer.
///
/// # Usage
/// ```rs
/// // To specify the client's IP and port for data transfer:
/// client.send_command("PORT h1,h2,h3,h4,p1,p2");
/// // Where h1-h4 are the octets of the client's IP address and p1, p2 are the high and low bytes of the port number (port = p1*256 + p2).
/// // For example, if the client's IP is 192.168.1.1 and the port is 21, the command would be:
/// // client.send_command("PORT 192,168,1,1,0,21");
/// // The server will respond with "200 PORT command successful" if the connection is established successfully.
/// // If the client is not authenticated, the server will respond with "530 Not logged in."
/// // If the server fails to connect to the client's specified address and port, it will respond with "425 Can't open data connection."
/// ```
#[allow(unused)]
pub struct CmdPort {
    ip: Ipv4Addr,
    port: u16,
}

impl FtpCommand for CmdPort {
    fn name() -> &'static str where Self: Sized { 
        "PORT" 
    }
}

impl Execute for CmdPort {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        check_auth!(client);

        let addr = SocketAddrV4::new(self.ip, self.port);
        
        match TcpStream::connect(addr) {
            Ok(stream) => {
                client.set_data(stream);
                SuccessResponse::Generic { 
                    details: "PORT command successful".to_string() 
                }.send(client.control())?;
            },
            Err(e) => {
                PermanentError::Generic { 
                    details: format!("Failed to connect to client: {}", e) 
                }.send(client.control())?;
            }
        }
        Ok(())
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdPort {
    fn build(arg: Option<String>) -> ftp_command_parser::Result<Box<dyn FtpCommandExec>> {
        let arg_str = tools::get_one_arg::<Self>(arg.clone())?;
        
        let parts: Vec<&str> = arg_str.split(',').collect();
        if parts.len() != 6 {
            return Err(ftp_command_parser::Error::UnvalidCmdArg { arg, code: Self::name().to_string() });
        }

        let nums: Result<Vec<u8>, _> = parts.iter().map(|s| s.parse::<u8>()).collect();
        let nums = nums.map_err(|_| ftp_command_parser::Error::UnvalidCmdArg { arg: arg.clone(), code: Self::name().to_string() })?;

        let ip = Ipv4Addr::new(nums[0], nums[1], nums[2], nums[3]);
        let port = (nums[4] as u16 * 256) + nums[5] as u16;

        Ok(Box::new(CmdPort { ip, port }))
    }
}