// 기본적인 베이스는 TCP를 활용한 바이트 통신과 HTTP를 이용한 요청과 응답

use std::io::{prelude::*, BufReader};
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// 연결 처리 위한 함수
fn handle_connection(mut stream: TcpStream) {
    // TcpStream으로부터 데이터를 읽고 출력
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}
