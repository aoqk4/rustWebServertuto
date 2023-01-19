// 기본적인 베이스는 TCP를 활용한 바이트 통신과 HTTP를 이용한 요청과 응답
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rust_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.excute(|| {
            handle_connection(stream);
        });
    }
}

// for multi webserver

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
}

// fn handle_connection(mut stream: TcpStream) {
//     // TcpStream으로부터 데이터를 읽고 출력 기본 형태
//     // let buf_reader = BufReader::new(&mut stream);
//     // let http_request: Vec<_> = buf_reader
//     //     .lines()
//     //     .map(|result| result.unwrap())
//     //     .take_while(|line| !line.is_empty())
//     //     .collect();

//     let buf_reader = BufReader::new(&mut stream);
//     let request_line = buf_reader.lines().next().unwrap().unwrap();

//     // 여러 입력 핸들링 위함
//     // if request_line == "GET / HTTP/1.1" {
//     //     let status_line = "HTTP/1.1 200 OK";
//     //     let contents = fs::read_to_string("hello.html").unwrap();
//     //     let length = contents.len();

//     //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//     //     stream.write_all(response.as_bytes()).unwrap();
//     // } else {
//     //     let status_line = "HTTP/1.1 404 NOT FOUND";
//     //     let contents = fs::read_to_string("404.html").unwrap();
//     //     let length = contents.len();

//     //     let response = format! {
//     //         "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
//     //     };

//     //     stream.write_all(response.as_bytes()).unwrap();
//     // }

//     // 핸들링 코드 간결하게
//     let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
//         ("HTTP/1.1 200 OK", "hello.html")
//     } else {
//         ("HTTP/1.1 404 NOT FOUND", "404.html")
//     };
//     let contents = fs::read_to_string(filename).unwrap();
//     let length = contents.len();

//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
//     stream.write_all(response.as_bytes()).unwrap();
// }
