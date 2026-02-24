//! Provides an explicit structure and its implementations
//! to handle the server-side model of an FtpClient.

use std::io::BufReader;
use std::net::TcpStream;

use data::DataType;
use crate::server::FtpServer;

pub mod data;
pub mod command;
pub mod run;

/// Represents the server-side state of an FtpClient
pub struct FtpClient<'a> {
    server: &'a FtpServer,
    control: TcpStream,
    
    /// A cloned read stream from control tcp stream to enable
    /// buffered read without having to recreate the bufreader for every iteration.
    control_read: BufReader<TcpStream>,
    
    /// The data stream for this client. <br/>
    /// It might not always be opened, hence the use of [`Option`]
    data: Option<TcpStream>,
    
    /// The format of data to be transfered over the data stream.
    pub data_type: DataType,

    pub username: String,

    /// Whether or not the user has fully authentified yet.
    pub authentified: bool,
}

impl<'a> FtpClient<'a> {
    pub fn new(control: TcpStream, server: &'a FtpServer) -> Self {
        Self {
            control: control.try_clone().unwrap(),
            control_read: BufReader::new(control),
            data: None,
            username: String::new(),
            authentified: false,
            data_type: DataType::Binary,
            server
        }
    }

    pub fn server(&self) -> &'a FtpServer {
        &self.server
    }

    pub fn control(&mut self) -> &mut TcpStream {
        &mut self.control
    }

    pub fn data(&mut self) -> Option<&mut TcpStream> {
        self.data.as_mut()
    }

    pub fn set_data(&mut self, stream: TcpStream) {
        self.data = Some(stream);
    }

    pub fn drop_data(&mut self) {
        self.data = None;
    }
    
    /// Retrieves both control and (option) data stream simultaneously.
    /// 
    /// This should be used when both are required in the same scope to comply with borrow rules.
    /// 
    /// Trying to access them through [`Self::control`] then [`Self::data`] in sequence as such a
    /// `control` borrow should remain alive when `data` is borrowed would imply two simultaneous
    /// mutable borrows of `self`. This is obviously not possible.
    pub fn streams(&mut self) -> (&mut TcpStream, Option<&mut TcpStream>) {
        (&mut self.control, self.data.as_mut())
    }
}
