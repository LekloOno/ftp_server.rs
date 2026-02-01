use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) {
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2121").unwrap();
    println!("FTP server listening on port 2121");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
