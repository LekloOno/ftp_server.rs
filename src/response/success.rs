use std::path::PathBuf;

use crate::response::FtpResponse;
use ftp_response_impl::FtpResponse;

#[derive(FtpResponse)]
pub enum SuccessResponse {
    #[ftp_res(code = 200, message = "{details}")]
    Generic { details: String },
    #[ftp_res(code = 220, message = "Service ready for new user. Welcome to the FTP server.")]
    ServiceReady,
    #[ftp_res(code = 227, message = "Entering passive mode ({h1},{h2},{h3},{h4},{p1},{p2})")]
    EnteringPasv { h1: u8, h2: u8, h3: u8, h4: u8, p1: u8, p2: u8 },
    #[ftp_res(code = 257, message = "{pathname:?}")]
    PathnameCreated { pathname: PathBuf },
    #[ftp_res(code = 230, message = "User logged in, proceed.")]
    UserLoggedIn,
    #[ftp_res(code = 250, message = "Requested file action okay, completed.")]
    DirectoryChanged,
    #[ftp_res(code = 226, message = "Closing data connection. Requested file action successful.")]
    ClosingDataConnection,
    #[ftp_res(code = 250, message = "File deleted successfully.")]
    FileDeleted,
    #[ftp_res(code = 250, message = "Directory removed successfully.")]
    DirectoryRemoved,
    #[ftp_res(code = 215, message = "UNIX Type: L8")]
    SystemType,
}