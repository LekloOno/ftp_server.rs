use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};
use crate::{check_auth, client::FtpClient, command::{Execute, FtpCommandExec, tools}};
use crate::response::{FtpResponse, SuccessResponse, PermanentError};

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
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        let arg_str = tools::get_one_arg::<Self>(arg.clone())?;
        
        let parts: Vec<&str> = arg_str.split(',').collect();
        if parts.len() != 6 {
            return Err(FtpParseError::UnvalidCmdArg { arg, code: Self::name().to_string() });
        }

        let nums: Result<Vec<u8>, _> = parts.iter().map(|s| s.parse::<u8>()).collect();
        let nums = nums.map_err(|_| FtpParseError::UnvalidCmdArg { arg: arg.clone(), code: Self::name().to_string() })?;

        let ip = Ipv4Addr::new(nums[0], nums[1], nums[2], nums[3]);
        let port = (nums[4] as u16 * 256) + nums[5] as u16;

        Ok(Box::new(CmdPort { ip, port }))
    }
}