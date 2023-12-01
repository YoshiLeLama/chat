use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufRead};

use chat_server::generate_auth_token;

fn handle_client(mut stream: TcpStream, auth_token: &str) {
    let sender_ip = stream.peer_addr().unwrap();
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let request_line = lines.next().expect("HTTP request message is empty").expect("Error reading the request line");

    println!("{}", request_line);

    match &request_line[..] {
        "AUTHENTICATE" => {
            if auth_token == lines.next().unwrap().unwrap() {
                stream.write(b"OK\n").unwrap();
            } else {
                stream.write(b"BAD TOKEN\n").unwrap();
            }
        }
        "MESSAGE" => {
            println!("{:?}: {}", sender_ip, lines.next().unwrap().unwrap());
        }
        _ => {
            stream.write(b"BAD REQUEST\n").unwrap();
        }
    }
    
    stream.flush().expect("Failed to flush the stream in handle_client");
}

fn main() -> std::io::Result<()> {
    let auth_token = generate_auth_token(32);
    println!("Auth token: {}", auth_token);

    let listener = TcpListener::bind("127.0.0.1:3030")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &auth_token);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
