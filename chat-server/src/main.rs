use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufRead};

static CONTENT: &str = "<html><head><meta charset=\"utf-8\"><title>Hello World</title></head><body><h1>Hello World</h1></body></html>";
static CONTENT_LENGTH: usize = CONTENT.len();

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().expect("HTTP request message is empty").expect("Error reading the request line");

    for word in request_line.split_whitespace() {
        println!("{}", word);
    }

    let message = format!("HTTP/1.1 200 OK\r\nContent-Length: {1}\r\n\r\n{0}", CONTENT, CONTENT_LENGTH);
    stream.write(message.as_bytes()).expect("Failed to write response in handle_client");
    stream.flush().expect("Failed to flush the stream in handle_client");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3030")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
