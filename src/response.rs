//! Describes FTP response in clear distinct categories.
//! 
//! This modules relies on [`ftp_response_impl`] to clearly and coherently
//! define and implement individual responses codes.

pub mod perma_error;
pub mod trans_error;
pub mod inter_reply;
pub mod preli_reply;
pub mod success;

pub use perma_error::PermanentError;
pub use trans_error::TransientError;
pub use inter_reply::IntermediateReply;
pub use preli_reply::PreliminaryReply;
pub use success::SuccessResponse;

pub trait FtpResponse {
    /// Sends the formated FtpResponse over the provided control stream
    fn send(&self, control: &mut dyn std::io::Write) -> std::io::Result<()>;
}