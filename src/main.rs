use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let parsed_request_line: Vec<&str> = request_line.split(" ").collect();

    let requested_path = parsed_request_line[1];

    let (status_line, filename) = if requested_path == "/" || requested_path == "/index.html" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    println!("{response}");
    let _ = stream.write_all(response.as_bytes());
}
