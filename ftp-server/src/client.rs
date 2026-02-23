use std::io::BufReader;
use std::net::TcpStream;

use crate::data::DataType;

pub mod command;
pub mod tools;

/// Represents the server-side state of an FtpClient
pub struct FtpClient {
    control: TcpStream,
    
    /// A cloned read stream from control tcp stream to enable
    /// buffered read without having to recreate the bufreader for every iteration.
    control_read: BufReader<TcpStream>,
    
    /// The data stream for this client. <br/>
    /// It might not always be opened, hence the use of [`Option`]
    pub data: Option<TcpStream>,
    
    /// The format of data to be transfered over the data stream.
    pub data_type: DataType,

    pub username: String,

    /// Whether or not the user has fully authentified yet.
    pub authentified: bool,

}

impl FtpClient {
    pub fn new(control: TcpStream) -> Self {
        Self {
            control: control.try_clone().unwrap(),
            control_read: BufReader::new(control),
            data: None,
            username: String::new(),
            authentified: false,
            data_type: DataType::Binary,
        }
    }

    pub fn control(&mut self) -> &mut TcpStream {
        &mut self.control
    }

    pub fn data(&mut self) -> Option<&mut TcpStream> {
        self.data.as_mut()
    }
}
