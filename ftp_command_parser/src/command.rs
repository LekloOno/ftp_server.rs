use crate::parser::FtpParseError;

pub trait FtpCommand {
    fn name() -> &'static str where Self: Sized;
}

pub trait BuildableFtpCommand<T: FtpCommand + ?Sized>: FtpCommand {
    fn build(arg: Option<String>) -> Result<Box<T>, FtpParseError>;
}