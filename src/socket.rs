use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};
use crate::registry::RouteRegistry;
use crate::util::extract_path;

//Creates client and establishes TCP listener on port 4000
pub async fn create_client(registry: &RouteRegistry) {
    let listener: TcpListener =  match TcpListener::bind("127.0.0.1:4000") {
        Ok(listener) => listener,
        Err(_error) => panic!("Could not create client!")
    };

    println!("\nLocal Host created on port 4000: http://localhost:4000");

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

//Handles the client, fetches responses that are written to stream
async fn handle_client(mut stream: &TcpStream, registry: &RouteRegistry) {
    let buf_reader = BufReader::new(stream);

    let pre_request_line =  match buf_reader.lines().next() {
        Some(request_line) => request_line,
        None => panic!("Could not parse string!")
    };

    let request_line = match pre_request_line {
        Ok(request) => request,
        Err(err) => panic!("Could not parse string: {}", err)
    };

    let path = extract_path(request_line);
    println!("{}", path);

    let response = registry.serve_route(&path);

    stream.write_all(response.as_bytes()).unwrap()
}


