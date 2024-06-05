use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).unwrap();

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 3 {
        
        return;
    }

    let method = parts[0];
    let target = parts[1]; 
    let version = parts[2];

    if method == "GET" && version == "HTTP/1.1" {
        let file_name = if target == "/" { "index.html" } else { &target[1..] };
        if fs::metadata(file_name).is_ok() {
            let contents = fs::read_to_string(file_name);
            match contents {
                Ok(contents) => {
                    let status_line = "HTTP/1.1 200 OK";
                    let length = contents.len();
                    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);
                    stream.write_all(response.as_bytes()).unwrap();
                },
                Err(_) => {
                    let status_line = "HTTP/1.1 403 FORBIDDEN";
                    let contents = "403 Forbidden";
                    let length = contents.len();
                    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);
                    stream.write_all(response.as_bytes()).unwrap()
                }
            }
        } else {
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let contents = "404 not found" ;
            let length = contents.len();
            let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);
            stream.write_all(response.as_bytes()).unwrap();
        }
    } else if method != "GET" {
        let status_line = "HTTP/1.1 405 METHOD NOT ALLOWED";
        let contents = "405 method not allowed" ;
        let length = contents.len();
        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);
        stream.write_all(response.as_bytes()).unwrap();
    }
}
