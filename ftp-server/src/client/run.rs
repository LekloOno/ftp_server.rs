use crate::{client::FtpClient, response::{FtpResponse, SuccessResponse}};

impl<'a> FtpClient<'a> {
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