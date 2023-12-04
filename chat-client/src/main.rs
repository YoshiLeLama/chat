use std::net::TcpStream;
use std::io::{Write, BufReader, BufRead, Error, ErrorKind};

fn authenticate(address: &str) -> Result<String, &str> {
    let mut auth_token = String::new();
    println!("Enter the authentication code:");
    std::io::stdin().read_line(&mut auth_token).unwrap();
    let auth_token = String::from(auth_token.trim());
    let auth_message = format!("AUTHENTICATE\n{}\n", auth_token);

    let mut stream = TcpStream::connect(format!("{0}:3030", address.trim())).unwrap();

    stream.write(auth_message.as_bytes()).unwrap();

    let reader = BufReader::new(&mut stream);
    let response = reader.lines().next().unwrap().unwrap();

    match &response[..] {
        "OK" => Ok(auth_token),
        "BAD TOKEN" => Err("The authentication token is incorrect"),
        _ => Err("Bad server response"),
    }
}

fn main() -> std::io::Result<()> {
    #[allow(unused_mut)]
    let mut address = String::from("127.0.0.1");
    // println!("Enter the address of the chat server:");
    // std::io::stdin().read_line(&mut address).unwrap();
   
    let auth_token;

    match authenticate(&address) {
        Ok(token) => {
            println!("Successfully authenticated");
            auth_token = token;
        }
        Err(e) => {
            eprintln!("Authentication error: {}", e);
            return Err(Error::from(ErrorKind::ConnectionRefused));
        }
    }

    loop {
        // Read message from the standard input
        print!("You > ");
        std::io::stdout().flush().unwrap();
        let mut message = String::new();
        std::io::stdin().read_line(&mut message).unwrap();
        let message = String::from(message.trim());

        if message == "!quit" {
            break;
        }
        
        // Send the message
        let mut stream = TcpStream::connect(format!("{0}:3030", address.trim()))?;
        let tcp_message = format!("MESSAGE\n{}\n{}\n", auth_token, message);
        stream.write(tcp_message.as_bytes())?;

        // Check if the message was successfully sent (if the token is correct)
        let reader = BufReader::new(&mut stream);
        let _response = reader.lines().next().unwrap().unwrap();
    }

    Ok(())
}
