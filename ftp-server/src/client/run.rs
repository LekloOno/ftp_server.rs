//! Provides methods to handle the main client listening loop of the server.

use crate::{client::FtpClient, response::{FtpResponse, SuccessResponse}};

impl<'a> FtpClient<'a> {
    /// Starts the client communication, and listens incoming commands.
    pub fn run(&mut self) -> crate::Result<()> {
        SuccessResponse::ServiceReady.send(&mut self.control())?;
    
        let mut line = String::new();

        loop {
            line.clear();
            let Some(command) = self.read_command(&mut line)? else {
                continue;
            };

            command.execute(self)?;
        }
    }
}