use std::net::TcpStream;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let mut address = String::new();
    println!("Enter the address of the chat server:");
    std::io::stdin().read_line(&mut address).unwrap();

    let mut stream = TcpStream::connect(format!("{0}:3030", address.trim()))?;

    stream.write(b"GET / HTTP/1.1\r\n\r\n")?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    println!("{}", response);

    Ok(())
}
