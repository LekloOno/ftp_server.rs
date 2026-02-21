use crate::response::FtpResponse;
use ftp_response_impl::{self, FtpResponse};

#[derive(FtpResponse)]
pub enum PermanentError {
    #[ftp_res(code = 500, message = "{details}")]
    Generic { details: String },
    #[ftp_res(code = 501, message = "Syntax error in parameters or arguments. {details}" )]
    ParamsArgsSyntax { details: String },
}