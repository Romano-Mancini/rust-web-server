use std::{
    fs,
    io::{BufRead, BufReader, Error, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result: Result<String, Error>| result.unwrap())
        .take_while(|line: &String| !line.is_empty())
        .collect();

    let request_path = http_request[0].split_whitespace().skip(1).next().unwrap();
    let (status_line, file_name) = match request_path {
        "/" => ("HTTP/1.1 200 OK", "index.html"),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sleep.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "not_found.html"),
    };
    let response = fs::read_to_string(file_name).unwrap();
    let length = response.len();

    stream
        .write(format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{response}").as_bytes())
        .unwrap();
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let thread_pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        thread_pool.assign(|| handle_connection(stream));
    }
}
