use ftp_command_parser::{
    parser::{FtpCommandParser, FtpParseError},
    command::FtpCommand,
    dummy_impl::{DisplayableFtpCommand, DummyFtpCommand}
};

pub fn main() -> Result<(), FtpParseError> {
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