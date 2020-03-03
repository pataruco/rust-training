use std::collections::VecDeque;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream, vec: &mut VecDeque<String>) -> Result<(), std::io::Error> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    stream.write_all(buffer.as_bytes())?;

    if !buffer.trim().is_empty() {
        println!("{} is stored 📬", buffer);
        vec.push_back(buffer);
    } else {
        println!("{}  not stored 📭 ", buffer);
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    let mut stored_messages: VecDeque<String> = VecDeque::new();

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?, &mut stored_messages)?;
    }
    Ok(())
}
