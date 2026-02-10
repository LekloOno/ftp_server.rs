use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {

    /// Username for FTP login - default: "anonymous"
    #[arg(short, long, default_value = "anonymous")]
    pub username: String,

    /// Password for FTP login - default: "anonymous"
    #[arg(short, long, default_value = "anonymous")]
    pub password: String,

    /// Port to listen on - default: 2121
    #[arg(short, long, default_value = "2121")]
    pub port: u16,

    /// Host to bind to - default: 127.0.0.1
    #[arg(default_value = "127.0.0.1")]
    pub host: String

}

