mod cli;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufRead, BufReader, Write};
use clap::Parser;
enum Command {
    User(String),
    Pass(String),
    Quit,
    List,
    Pwd,
    Unknown(String)
}

fn handle_client(mut stream: TcpStream) {
    stream.write_all(b"220 Welcome to the FTP server\r\n").unwrap();

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();

    loop {
        line.clear();
        if reader.read_line(&mut line).unwrap() == 0 { break; }
        let cmd = parse_command(&line);

        match cmd {
            Command::User(name) => {
                println!("User: {}", name);
                stream.write_all(b"331 Password required\r\n").unwrap();
            },
            Command::Pass(_) => {
                stream.write_all(b"230 User logged in\r\n").unwrap();
            },
            Command::Pwd => {
                stream.write_all(b"257 \"/\" is current directory\r\n").unwrap();
            },
            Command::Quit => {
                stream.write_all(b"221 Goodbye\r\n").unwrap();
                break;
            },
            _ => {
                stream.write_all(b"500 Unknown command\r\n").unwrap();
            }
        }
    }
}

fn parse_command(line: &str) -> Command {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Command::Unknown("".to_string());
    }
    match parts[0].to_uppercase().as_str() {
        "USER" => Command::User(parts.get(1).unwrap_or(&"").to_string()),
        "PASS" => Command::Pass(parts.get(1).unwrap_or(&"").to_string()),
        "QUIT" => Command::Quit,
        "LIST" => Command::List,
        "PWD"  => Command::Pwd,
        _ => Command::Unknown(parts[0].to_string()),
    }
}

fn main() {
    let args = cli::Cli::parse();

    let addr = format!("{}:{}", args.host, args.port);

    let listener = TcpListener::bind(&addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {handle_client(stream)});
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

}
