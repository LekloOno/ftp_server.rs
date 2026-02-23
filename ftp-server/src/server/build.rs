use std::{collections::HashMap, path::PathBuf};

use ftp_command_parser::{command::BuildableFtpCommand, parser::FtpCommandParser};

use crate::{command::FtpCommandExec, server::FtpServer};

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

    pub fn register_command<B>(mut self) -> Self
    where
        B: BuildableFtpCommand<dyn FtpCommandExec> + 'static
    {
        self.parser.register::<B>();
        self
    }

}