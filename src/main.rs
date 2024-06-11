mod router;

use crate::router::swap_to_custom_route;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{thread, fs};

fn get_page(path: &str) -> String {
    let file_path = if !path.split('/').last().unwrap_or("").contains('.') {
        "public".to_owned() + path + "/index.html"
    } else {
        "public".to_owned() + path
    };

    match fs::read_to_string(&file_path) {
        Ok(file) => file,
        Err(_) => fs::read_to_string("public/404/index.html").expect("404 page not found"),
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);

    let mut path = request.split_whitespace().nth(1).unwrap();
    path = swap_to_custom_route(path);

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n".to_owned() + &get_page(path);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => { 
                println!("Failed: {}", e);
            }
        }
    }
}