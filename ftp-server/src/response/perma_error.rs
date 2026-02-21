use crate::response::FtpResponse;
use ftp_response_impl::{self, FtpResponse};

#[derive(FtpResponse)]
pub enum PermanentError {
    #[ftp_res(code = 500, message = "{details}")]
    Generic { details: String },
    #[ftp_res(code = 501, message = "Syntax error in parameters or arguments. {details}" )]
    ParamsArgsSyntax { details: String },
    #[ftp_res(code = 502, message = "Command not implemented. {unimpl_code}")]
    Unimplemented { unimpl_code: String },
    #[ftp_res(code = 504, message = "Command not implemented for that parameter. {unimpl_param}")]
    UnimplParameter { unimpl_param: String },
}