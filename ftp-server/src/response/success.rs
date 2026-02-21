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
}

#[macro_export]
macro_rules! send_ok {
    ($ctrl:expr) => {
        $control.write("200 Ok !")
    };
    ($ctrl:expr, $fmt:literal $(, $($arg:tt)*)?) => {
        write!($ctrl, $fmt $(, $($arg)*)?)
    };
}