use crate::command::{BuildableFtpCommand, FtpCommand};

pub struct DummyFtpCommand {
    arg: Option<String>,
}

impl FtpCommand for DummyFtpCommand {
    fn name() -> &'static str where Self: Sized {
        "DUM"
    }
}

impl BuildableFtpCommand<dyn DisplayableFtpCommand> for DummyFtpCommand {
    fn build(arg: Option<String>) -> crate::Result<Box<dyn DisplayableFtpCommand>> {
        Ok(Box::new(Self { arg }))
    }
}

impl std::fmt::Display for DummyFtpCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", Self::name(), self.arg.as_ref().map_or("nothing !", String::as_str))
    }
}

pub trait DisplayableFtpCommand : FtpCommand + std::fmt::Display {}

impl<T> DisplayableFtpCommand for T where T: FtpCommand + std::fmt::Display {}