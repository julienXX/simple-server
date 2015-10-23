use std::net::{TcpListener, TcpStream};
use std::io::{Write};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Listening for connections on port {}", 8080u32);

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream, String::from("<h1>Welcome to Nginx</h1><br />trololo"))
                });
            }
            Err(_) => { /* connection failed */ }
        }
    }
    drop(listener);
}

fn handle_client(mut stream: TcpStream, text: String) {
    let response = response_for(text);
    stream.write(&response.into_bytes()).unwrap();
}

fn response_for(body: String) -> String {
    format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\ncontent-length: {}\r\n\r\n{}", body.len(), body)
}
