mod command;
mod cli;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufRead, BufReader, Write};
use clap::Parser;

use ftp_command_parser::parser::FtpCommandParser;
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

fn handle_client(stream: TcpStream, parser: std::sync::Arc<FtpCommandParser<dyn FtpCommandExec>>) {
    let mut stream_writer = stream.try_clone().expect("Failed to clone stream");
    stream_writer.write_all(b"220 Welcome to the FTP server\r\n").unwrap();

    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();
        if reader.read_line(&mut line).unwrap() == 0 { break; }
        
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }

        let mut parts = trimmed.splitn(2, ' ');
        let code = parts.next().unwrap_or("").to_uppercase();
        let arg = parts.next().map(|s| s.to_string());

        let code_static: &'static str = Box::leak(code.into_boxed_str());

        match parser.get(code_static) {
            Ok(builder) => {
                match builder.build(arg) {
                    Ok(command) => {
                        command.execute(&mut stream_writer);
                        if code_static == "QUIT" {
                            break;
                        }
                    }
                    Err(e) => {
                        let msg = format!("501 Syntax error in parameters or arguments: {}\r\n", e);
                        stream_writer.write_all(msg.as_bytes()).unwrap();
                    }
                }
            },
            Err(_) => {
                stream_writer.write_all(b"500 Unknown command\r\n").unwrap();
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

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let p = std::sync::Arc::clone(&parser);
                thread::spawn(move || {
                    handle_client(stream, p);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}