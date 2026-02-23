use std::{collections::HashMap, path::PathBuf};

use ftp_command_parser::parser::FtpCommandParser;

use crate::command::FtpCommandExec;

pub mod build;
pub mod run;

pub struct FtpServer {
    pub hostname: String,
    pub port: u16,
    /// user/password map. <br/>
    /// a `None` password means this user does not require any authentication.
    /// Typically used for a default user.
    /// 
    /// # Notes
    /// This is a placeholder for simplicity and illustration purpose.
    /// 
    /// In an actual server, we would store such in a database or
    /// configuration file, not within the structure itself, and only then
    /// query it.
    /// 
    /// Also, the password is stored in plain text, but we'd obviously
    /// rather store it hashed in real use. 
    pub users: HashMap<String, Option<String>>,
    pub root_path: PathBuf,
    pub protect_root: bool,
    pub parser: FtpCommandParser<dyn FtpCommandExec>,
}

impl FtpServer {
    pub fn new() -> Self {
        Self {
            hostname: String::new(),
            port: 0,
            users: HashMap::new(),
            root_path: PathBuf::new(),
            protect_root: true,
            parser: FtpCommandParser::<dyn FtpCommandExec>::new(),
        }
    }
}