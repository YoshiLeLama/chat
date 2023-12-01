use std::net::TcpStream;
use std::io::{Write, BufReader, BufRead, Error, ErrorKind};

fn authenticate(address: &str) -> Result<(), &str> {
    let mut auth_token = String::new();
    println!("Enter the authentication code:");
    std::io::stdin().read_line(&mut auth_token).unwrap();
    let auth_token = auth_token.trim();
    let auth_message = format!("AUTHENTICATE\n{}\n", auth_token);

    let mut stream = TcpStream::connect(format!("{0}:3030", address.trim())).unwrap();

    stream.write(auth_message.as_bytes()).unwrap();

    let reader = BufReader::new(&mut stream);
    let response = reader.lines().next().unwrap().unwrap();

    match &response[..] {
        "OK" => Ok(()),
        "BAD TOKEN" => Err("The authentication token is incorrect"),
        _ => Err("Bad server response"),
    }
}

fn main() -> std::io::Result<()> {
    let mut address = String::from("127.0.0.1");
    // println!("Enter the address of the chat server:");
    // std::io::stdin().read_line(&mut address).unwrap();
    
    match authenticate(&address) {
        Ok(_) => println!("Successfully authenticated"),
        Err(e) => {
            eprintln!("Authentication error: {}", e);
            return Err(Error::from(ErrorKind::ConnectionRefused));
        }
    }

    {
        let mut stream = TcpStream::connect(format!("{0}:3030", address.trim()))?;

        stream.write("MESSAGE\nsalut à tous les amis\n".as_bytes())?;
    }

    Ok(())
}
