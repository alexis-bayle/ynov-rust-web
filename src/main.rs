use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    env,
};

fn main() {
    const DEFAULT_PORT: &str = "7878";
    let port = env::var("PING_LISTEN_PORT").unwrap_or(DEFAULT_PORT.to_string());

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let route = http_request[0].split_whitespace().collect::<Vec<&str>>()[1];

    if route == "/ping" {
        let response = format!("HTTP/1.1 200\r\n\r\n{http_request:#?}");
        stream.write_all(response.as_bytes()).unwrap();
        return;
    } else {
        let response = format!("HTTP/1.1 404\r\n\r\n");
        stream.write_all(response.as_bytes()).unwrap();
        return;
    }
}
