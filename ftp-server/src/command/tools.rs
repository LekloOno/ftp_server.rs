//! Provides tools to handle common operation within commands
//! and simplify their implementation process.

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
    let Some(arg_str) = &arg else {
        return Err(FtpParseError::UnvalidCmdArg { arg, code: T::name().to_string() });
    };
    
    // We could be smarter and iterate then stop once we have more than one args
    // This would avoid potential expensive collect .. Probably negligeable anyways
    let args = arg_str.split_whitespace().collect::<Vec<&str>>();

    if args.len() != 1 {
        return Err(FtpParseError::UnvalidCmdArg { arg, code: T::name().to_string() });
    }

    Ok(args[0].to_string())
}

/// Checks if the provided arg option contains exactly one argument.
/// 
/// The difference with [`get_one_arg`] is that the relaxed version allows separator chars within the argument.<br/>
/// This means it will only try to split the provided argument string once, at the first separator, and the other
/// separators will be considered to be part of the provided single argument.
/// 
/// This typically allows to have separator chars within such arg itself, which is necessary for file names for example.
/// 
/// # Returns
/// If it contains exactly one string argument (that might include separator chars), returns such argument.
pub fn get_one_arg_relaxed<T: FtpCommand>(arg: Option<String>) -> Result<String, FtpParseError> {
    let Some(arg_str) = &arg else {
        return Err(FtpParseError::UnvalidCmdArg { arg, code: T::name().to_string() });
    };
    
    let Some((_, r_arg)) = arg_str.split_once(char::is_whitespace) else {
        return Err(FtpParseError::UnvalidCmdArg { arg, code: T::name().to_string() });
    };

    Ok(r_arg.to_string())
}

/// A quick hand to simplify implementations of commands that require authentication.
/// 
/// It checks if the provided client is authentified, and if not, sends a `NotLoggedIn`
/// response through the control stream and early returns `Ok(())`.
#[macro_export]
macro_rules! check_auth {
    ($client:expr) => {
        if !$client.authentified {
            crate::response::perma_error::PermanentError::NotLoggedIn { details: String::new() }.send($client.control())?;
            return Ok(());
        }
    };
}

/// A quick helper to send a generic Ok response, with an optionnal custom message
/// 
/// # Usage
/// ```rs
/// send_ok!(client.control());
/// // Equivalent to
/// SuccessResponse::Generic { details: "Ok !".to_string() }
///     .send(client.control);
/// // Or
/// client.control.write("200 Ok !");
/// 
/// let name = "big chungus";
/// let count = 3;
/// send_ok!(client.control(), "Sucessfully summoned {count} {name} demon(s)")
/// // Equivalent to (without string allocation !)
/// SuccessResponse::Generic { details: format!("Successfully summoned {count} {name} demon(s)" }
///     .send(client.control);
/// // or
/// write!(client.control, "200 Successfully summoned {count} {name} demon(s)");
/// ```
#[macro_export]
macro_rules! send_ok {
    ($ctrl:expr) => {
        SuccessResponse::Generic { details: "Ok !".to_string() }.send($ctrl)
    };
    ($ctrl:expr, $fmt:literal $(, $($arg:tt)*)?) => {
        SuccessResponse::Generic { details: format!($fmt $(, $($arg)*)?) }.send($ctrl)
    };
}