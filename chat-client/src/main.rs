use std::net::TcpStream;
use std::io::{Write, BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let mut address = String::from("127.0.0.1");
    // println!("Enter the address of the chat server:");
    // std::io::stdin().read_line(&mut address).unwrap();

    let mut auth_token = String::new();
    println!("Enter the authentication code:");
    std::io::stdin().read_line(&mut auth_token).unwrap();
    let auth_token = auth_token.trim();
    let auth_message = format!("AUTHENTICATE\n{}\n", auth_token);

    let mut stream = TcpStream::connect(format!("{0}:3030", address.trim()))?;

    stream.write(auth_message.as_bytes())?;

    let reader = BufReader::new(&mut stream);
    let response = reader.lines().next().unwrap().unwrap();
    println!("{}", response);

    Ok(())
}
