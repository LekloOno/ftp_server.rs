mod command;
mod cli;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufRead, BufReader};
use clap::Parser;

use ftp_command_parser::parser::FtpCommandParser;
use ftp_server::response;
use crate::response::*;

use crate::command::FtpCommandExec;
use crate::command::commands::{
    user::CmdUser, pass::CmdPass, pwd::CmdPwd, cwd::CmdCwd, 
    pasv::CmdPasv, cdup::CmdCdup, list::CmdList, quit::CmdQuit
};

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

fn handle_client(control: TcpStream, parser: std::sync::Arc<FtpCommandParser<dyn FtpCommandExec>>) {
    // Cloned to enable buffered reader, without having to recreate the bufreader for every write iteration
    let mut control_res = control.try_clone().expect("Failed to clone stream");
    SuccessResponse::ServiceReady.send(&mut control_res).unwrap();
    
    let mut reader = BufReader::new(&control);
    let mut line = String::new();

    loop {
        line.clear();
        if reader.read_line(&mut line).unwrap() == 0 { break; }
        
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }

        let mut parts = trimmed.splitn(2, ' ');
        let code = parts.next().unwrap_or("").to_uppercase();
        let arg = parts.next().map(|s| s.to_string());

        match parser.get(&code) {
            Ok(builder) => {
                match builder.build(arg) {
                    Ok(command) => {
                        command.execute(&mut control_res);
                        if code == "QUIT" {
                            break;
                        }
                    }
                    Err(e) => {
                        PermanentError::ParamsArgsSyntax { details: e.to_string() }
                            .send(&mut control_res).unwrap();
                    }
                }
            },
            Err(_) => {
                PermanentError::Generic { details: "Unknown command".to_string() }
                    .send(&mut control_res).unwrap();
            }
        }
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
                    handle_client(control, p);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}