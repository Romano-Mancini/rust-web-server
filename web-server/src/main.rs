use std::{
    fs,
    io::{BufRead, BufReader, Error, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use web_server::ThreadPool;

const ADDRESS: &str = "127.0.0.1:8080";

fn send_response(status_line: &str, file_name: &str, stream: &mut TcpStream) {
    let response = fs::read_to_string(file_name).unwrap();
    let length = response.len();

    stream
        .write(format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{response}").as_bytes())
        .unwrap();
}

fn handle_connection(mut stream: TcpStream, shutdown: Arc<Mutex<bool>>) {
    let buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result: Result<String, Error>| result.unwrap())
        .take_while(|line: &String| !line.is_empty())
        .collect();

    let request_path = http_request[0].split_whitespace().skip(1).next().unwrap();
    match request_path {
        "/" => {
            send_response("HTTP/1.1 200 OK", "index.html", &mut stream);
        }
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            send_response("HTTP/1.1 200 OK", "sleep.html", &mut stream);
        }
        "/kill" => {
            send_response("HTTP/1.1 200 OK", "kill.html", &mut stream);
            thread::sleep(Duration::from_secs(5));
            *shutdown.lock().unwrap() = true;
            TcpStream::connect(ADDRESS).unwrap();
        }
        _ => {
            send_response("HTTP/1.1 404 NOT FOUND", "not_found.html", &mut stream);
        }
    }
}

fn main() {
    let listener: TcpListener = TcpListener::bind(ADDRESS).unwrap();
    let thread_pool = ThreadPool::new(5);
    let shutdown = Arc::new(Mutex::new(false));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        if *shutdown.lock().unwrap() {
            break;
        }
        let clone = Arc::clone(&shutdown);
        thread_pool.assign(|| handle_connection(stream, clone));
    }

    println!("Shutting down.");
}
