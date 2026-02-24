//! Defines executable, bindable and parseable independant commands
//! implementations for an [`crate::client::FtpClient`].

pub mod user;  /// The `USER` command, which is used for user authentication.
pub mod pass;  /// The `PASS` command, which is used for user authentication.
pub mod pwd;  /// The `PWD` command, which returns the current working directory of the client on the server.
pub mod cwd;  /// The `CWD` command, which changes the current working directory of the client on the server.
pub mod pasv;  /// The `PASV` command, which puts the server into passive mode for data transfer.
pub mod cdup;  /// The `CDUP` command, which changes the current working directory to the parent directory on the server.
pub mod list;  /// The `LIST` command, which lists the files and directories in the current working directory on the server.
pub mod quit;  /// The `QUIT` command, which terminates the FTP session with the server.
pub mod dtype;  /// The `TYPE` command, which sets the data transfer type (e.g., ASCII or binary) for file transfers.
pub mod retr;  /// The `RETR` command, which retrieves a file from the server to the client.
pub mod stor;  /// The `STOR` command, which stores a file from the client to the server.
pub mod dele;  /// The `DELE` command, which deletes a file on the server.
pub mod mkd;  /// The `MKD` command, which creates a new directory on the server.
pub mod rmd;  /// The `RMD` command, which removes a directory on the server.
pub mod syst;  /// The `SYST` command, which returns the type of operating system the server is running on.
pub mod port;  /// The `PORT` command, which allows the client to specify an IP address and port for the server to connect to for data transfer.
pub mod rest;  /// The `REST` command, which allows the client to specify a byte offset for resuming file transfers.