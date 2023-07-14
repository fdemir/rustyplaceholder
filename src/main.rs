use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Duration,
};

fn handle_connection(mut stream: TcpStream) {
    // let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let content = "<body bgcolor='black'><h1 style='color: pink;'>hello kity!</h1></body>";
    let length = content.len();

    let response: String = format!("HTTP/1.1 200 OK\r\nContent-Length: {length} \r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}
