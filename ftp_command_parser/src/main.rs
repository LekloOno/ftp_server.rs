//! A simple program showcasing the use of the parser with [`ftp_command_parser::dummy_impl`].
use ftp_command_parser::{
    Result,
    parser::FtpCommandParser,
    command::FtpCommand,
    dummy_impl::{DisplayableFtpCommand, DummyFtpCommand}
};

pub fn main() -> Result<()> {
    let mut command_parser = FtpCommandParser::<dyn DisplayableFtpCommand>::new();
    command_parser.register::<DummyFtpCommand>();

    let code = DummyFtpCommand::name();
    let arg = Some("geda gedee geda geda ooo".to_string());

    let command = command_parser
        .get(code)?
        .build(arg)?;

    println!("{command}");

    Ok(())
}