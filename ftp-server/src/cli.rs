//! Defines the ftp-server command arguments
 
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {

    /// Username for FTP login - default: "anonymous"
    #[arg(short, long, default_value = "anonymous")]
    pub username: String,

    /// Password for FTP login - default: No password
    #[arg(short, long)]
    pub password: Option<String>,

    /// Port to listen on - default: 2121
    #[arg(short='P', long, default_value = "2121")]
    pub port: u16,

    /// Disables root protection, and allows users from traversing parent directories of the `root_path`.
    #[arg(short='t', long)]
    pub allow_traversal: bool,

    /// Host to bind to - default: 127.0.0.1
    #[arg(short='H', long, default_value = "127.0.0.1")]
    pub host: String,

    /// Sets the initial root directory to serve
    #[arg(default_value = ".")]
    pub root_path: PathBuf,
}

