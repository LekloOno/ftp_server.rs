use std::io::BufRead;

use ftp_command_parser::{command::FtpCommand, parser::FtpCommandParser};

use crate::{client::FtpClient, response::*};
use crate::command::Execute;

impl FtpClient {
    /// Reads the control stream through the buffered reader, and tries to parse its content into a command.
    /// 
    /// # Returns
    /// * `Some(Box<T>)` - If the next line could be parsed to such an [`FtpCommand`].
    /// * `None` - If the next line did not correspond to any parseable command.
    /// 
    /// # Params
    /// * `buf` - a pre-allocated buffer to avoid the repeated allocation cost
    /// * `parser` - the command parser to use
    /// 
    /// # Errors
    /// The method handles all expected Ftp errors internally - that is, parsing errors dued to
    /// unimplemented codes, malformed arguments and parameters as well as syntax errors - by sending
    /// the appopriated responses over the client's control stream.
    /// 
    /// The only errors this method can return are purely I/O related errors.
    pub fn read_command<T: FtpCommand + Execute + ?Sized>(
        &mut self,
        buf: &mut String,
        parser: &FtpCommandParser<T>
    ) -> std::io::Result<Option<Box<T>>> {    
        buf.clear();
        if self.control_read.read_line(buf).unwrap() == 0 {
            return Ok(None)
        };

        let trimmed = buf.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }

        let mut parts = trimmed.splitn(2, ' ');

        let code = parts
            .next()
            .unwrap_or("")
            .to_uppercase();
        
        let Ok(builder) = parser.get(&code) else {
            PermanentError::Unimplemented { unimpl_code: code.to_string() }
                .send(&mut self.control)?;
            return Ok(None);
        };
    
        let arg = parts
            .next()
            .map(ToString::to_string);

        match builder.build(arg) {
            Ok(command) =>
                Ok(Some(command)),
            Err(e) => {
                PermanentError::ParamsArgsSyntax { details: e.to_string() }
                    .send(&mut self.control())?;
                Ok(None)
            }
        }
    }
}