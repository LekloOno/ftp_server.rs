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
    fn send(&self, control: &mut dyn std::io::Write) -> std::io::Result<()>;
}