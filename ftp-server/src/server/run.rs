use std::{net::TcpListener, thread};


use crate::{client::FtpClient, server::FtpServer};

impl FtpServer {
    pub fn run(&self) -> crate::Result<()> {
        let addr = format!("{}:{}", self.hostname, self.port);

        let listener = TcpListener::bind(&addr)?;
        println!("FTP Server listening on {}", addr);
        
        thread::scope(|s| {
            for control in listener.incoming() {
                match control {
                    Ok(control) => {
                        s.spawn(move || {
                            let _ = FtpClient::new(control, self).run();
                        });
                    }
                    Err(e) => {
                        eprintln!("Connection failed: {}", e);
                    }
                }
            }
        });

        Ok(())
    }
}