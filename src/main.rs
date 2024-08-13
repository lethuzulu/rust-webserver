use std::{
    alloc::handle_alloc_error,
    fmt::format,
    io::{BufRead, BufReader, BufWriter, Write},
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
    let mut buf_reader = BufReader::new(&stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let parsed_request_line: Vec<&str> = request_line.split(" ").collect();

    let status_line = "HTTP/1.1 200 OK";

    let requested_path = parsed_request_line[1];

    let response = format!("{status_line}\r\n\r\nRequested path: {requested_path}");

    stream.write_all(response.as_bytes());
}
