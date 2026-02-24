use crate::response::FtpResponse;
use ftp_response_impl::{self, FtpResponse};

#[derive(FtpResponse)]
pub enum PreliminaryReply {
    #[ftp_res(code = 150, message = "File status okay; about to open data connection.")]
    AboutToOpenDataConnection,
}