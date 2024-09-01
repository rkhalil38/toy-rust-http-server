use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};
use crate::registry::RouteRegistry;
use crate::util::extract_path;


pub async fn create_client(registry: &RouteRegistry) {
    let listener: TcpListener =  TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(&stream, registry).await
            },
            Err(err) => {
                print!("COULD NOT CREATE CLIENT: {}", err)
            }
        }
    }
}

async fn handle_client(mut stream: &TcpStream, registry: &RouteRegistry) {
    let buf_reader = BufReader::new(stream);

    let request_line: String = buf_reader.lines().next().unwrap().unwrap();
    let path = extract_path(request_line);
    println!("{}", path);

    let response = registry.serve_route(&path);

    stream.write_all(response.as_bytes()).unwrap()
}


