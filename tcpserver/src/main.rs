use std::net::TcpListener;
use std::io::{Read, Write};
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("received from client {:?}", str::from_utf8(&buffer).unwrap());
        stream.write(&mut buffer).unwrap();
        
    }
} 