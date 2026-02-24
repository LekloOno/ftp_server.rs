//! The fundamental traits to define commands that can be built
//! by a parser.

/// Identifies a command via a string name code.
pub trait FtpCommand {
    fn name() -> &'static str where Self: Sized;
}

/// An extension helper trait to retrieve a command name
/// via dynamic dispatch.
/// 
/// It is implemented through a strandard blanket
/// implementation for every FtpCommand implementers.
pub trait FtpCommandDyn: FtpCommand {
    fn name(&self) -> &'static str;
}

impl<T: FtpCommand> FtpCommandDyn for T {
    fn name(&self) -> &'static str {
        T::name()
    }
}

/// Specifies how the implementing command should be built for the
/// target dynamic type `T`.
pub trait BuildableFtpCommand<T: FtpCommand + ?Sized>: FtpCommand {
    fn build(arg: Option<String>) -> crate::Result<Box<T>>;
}