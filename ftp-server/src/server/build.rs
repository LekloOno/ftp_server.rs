//! Provides methods to configure a server in a Builder pattern. 
 
use std::{collections::HashMap, path::PathBuf};
use ftp_command_parser::{command::BuildableFtpCommand, parser::FtpCommandParser};
use crate::{command::FtpCommandExec, server::FtpServer};

use std::fs::File;
use std::io::{BufRead, BufReader};

impl FtpServer {
    pub fn hostname(mut self, hostname: String) -> Self {
        self.hostname = hostname;
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn users(mut self, users: HashMap<String, Option<String>>) -> Self {
        self.users = users;
        self
    }

    pub fn root_path(mut self, root_path: PathBuf) -> Self {
        self.root_path = root_path;
        self
    }

    pub fn protect_root(mut self, protect_root: bool) -> Self {
        self.protect_root = protect_root;
        self
    }

    pub fn parser(mut self, parser: FtpCommandParser<dyn FtpCommandExec>) -> Self {
        self.parser = parser;
        self
    }

    pub fn add_user(mut self, user: String, pwd: Option<String>) -> Self {
        // Would need to handle the case where the user exists -
        // in such a case, we would need to make sure the client requesting
        // is the corresponding user, and is authentified.
        // Yet, this might not be the responsibility of add_user method anyways.
        self.users.insert(user, pwd);
        self
    }

    /// Registers the `B` command to this server's parser so that it can be recognized.
    pub fn register_command<B>(mut self) -> Self
    where
        B: BuildableFtpCommand<dyn FtpCommandExec> + 'static
    {
        self.parser.register::<B>();
        self
    }

    pub fn load_users_from_file(mut self, filepath: Option<PathBuf>) -> Self {
        let Some(path) = filepath else {
            return self;
        };

        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    self.users.insert(parts[0].to_string(), Some(parts[1].to_string()));
                } 
                else if parts.len() == 1 && !parts[0].is_empty() {
                    self.users.insert(parts[0].to_string(), None);
                }
            }
            println!("Loaded users from {:?}", path);
        } 
        else {
            eprintln!("Warning: Could not open users file at {:?}", path);
        }
        self
    }

}