use ftp_command_parser::{command::{BuildableFtpCommand, FtpCommand}, parser::FtpParseError};

use crate::{client::FtpClient, command::{Execute, FtpCommandExec, tools}, data::DataType, response::{FtpResponse, PermanentError, SuccessResponse}};
use std::io::Write;

#[allow(unused)]
pub struct CmdType {
    data_type: DataType,
}

impl FtpCommand for CmdType {
    fn name() -> &'static str where Self: Sized {
        "TYPE"
    }
}

impl Execute for CmdType {
    fn execute(&self, client: &mut FtpClient) -> crate::Result<()> {
        let type_str = self.data_type.to_string();
        if let DataType::Ebcdic = self.data_type {
            PermanentError::UnimplParameter { unimpl_param: type_str }
                .send(client.control())?;
            return Ok(());
        }

        client.data_type = self.data_type.clone();
        SuccessResponse::Generic { 
            details: format!("Type set to {}", type_str) 
        }
        .send(client.control())
        .map_err(crate::Error::from)
    }
}

impl BuildableFtpCommand<dyn FtpCommandExec> for CmdType {
    fn build(arg: Option<String>) -> Result<Box<dyn FtpCommandExec>, FtpParseError> {
        let arg_str = tools::get_one_arg::<Self>(arg)?;

        let data_type = DataType::try_from(arg_str.as_str())
            .map_err(|_| FtpParseError::UnvalidCmdArg {
                arg: Some(arg_str),
                code: Self::name().to_string()
            })?;

        Ok(Box::new(CmdType {data_type}))
    }
}