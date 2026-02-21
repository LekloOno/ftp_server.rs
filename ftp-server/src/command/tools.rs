use ftp_command_parser::{command::FtpCommand, parser::FtpParseError};

/// Checks if the provided arg is empty.
/// Use it to propagate the error otherwise.
/// 
/// # Note
/// The function consumes the argument, as such argument contains 
/// no information anyway if it does not produce an error.
pub fn get_no_args<T: FtpCommand>(arg: Option<String>) -> Result<(), FtpParseError> {
    let Some(arg_str) = &arg else {
        return Ok(());
    };

    if arg_str.is_empty() {
        return Ok(());
    }

    Err(FtpParseError::UnvalidCmdArg { arg, code: T::name().to_string() })
}

/// Checks if the provided arg option contains exactly one argument.
/// That is, if the arg option is a Some, and its content contains multiple parts once splitted by the common separator.
/// 
/// # Returns
/// If it contains exactly one string argument, returns such argument.
pub fn get_one_arg<T: FtpCommand>(arg: Option<String>) -> Result<String, FtpParseError> {
    let Some(password) = &arg else {
        return Err(FtpParseError::UnvalidCmdArg { arg, code: T::name().to_string() });
    };
    
    // We could be smarter and iterate then stop once we have more than one args
    // This would avoid potential expensive collect .. Probably negligeable anyways
    let args = password.split_whitespace().collect::<Vec<&str>>();

    if args.len() != 1 {
        return Err(FtpParseError::UnvalidCmdArg { arg, code: T::name().to_string() });
    }

    Ok(args[0].to_string())
}