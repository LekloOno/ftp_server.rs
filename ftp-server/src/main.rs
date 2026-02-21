mod cli;

use std::net::{TcpListener, TcpStream};
use std::thread;
use clap::Parser;

use ftp_command_parser::parser::FtpCommandParser;
use ftp_server::client::FtpClient;
use ftp_server::response::*;

use ftp_server::command::FtpCommandExec;
use ftp_server::command::commands::{
    user::CmdUser, pass::CmdPass, pwd::CmdPwd, cwd::CmdCwd, 
    pasv::CmdPasv, cdup::CmdCdup, list::CmdList, quit::CmdQuit
};

use ftp_server::Result;

fn create_parser() -> FtpCommandParser<dyn FtpCommandExec> {
    let mut parser = FtpCommandParser::<dyn FtpCommandExec>::new();
    
    parser.register::<CmdUser>();
    parser.register::<CmdPass>();
    parser.register::<CmdPwd>();
    parser.register::<CmdCwd>();
    parser.register::<CmdPasv>();
    parser.register::<CmdCdup>();
    parser.register::<CmdList>();
    parser.register::<CmdQuit>();
    
    parser
}

fn handle_client(control: TcpStream, parser: std::sync::Arc<FtpCommandParser<dyn FtpCommandExec>>) -> Result<()> {
    let mut client = FtpClient::new(control);
    SuccessResponse::ServiceReady.send(&mut client.control())?;
    
    let mut line = String::new();

    loop {
        line.clear();
        let Some(command) = client.read_command(&mut line, parser.as_ref())? else {
            continue;
        };

        command.execute(&mut client)?;
    }
}

fn main() {
    let args = cli::Cli::parse();
    let addr = format!("{}:{}", args.host, args.port);

    let parser = std::sync::Arc::new(create_parser());

    let listener = TcpListener::bind(&addr).unwrap();
    println!("FTP Server listening on {}", addr);
    
    for control in listener.incoming() {
        match control {
            Ok(control) => {
                let p = std::sync::Arc::clone(&parser);
                thread::spawn(move || {
                    let _ = handle_client(control, p);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}