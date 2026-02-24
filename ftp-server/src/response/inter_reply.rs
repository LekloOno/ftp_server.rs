use crate::response::FtpResponse;
use ftp_response_impl::{self, FtpResponse};

#[derive(FtpResponse)]
pub enum IntermediateReply {
    #[ftp_res(code = 331, message = "User name okay, password needed")]
    UserNameOk,
    #[ftp_res(code = 350, message = "Restart position accepted ({offset}).")]
    RestartMarker { offset: u64 },
}