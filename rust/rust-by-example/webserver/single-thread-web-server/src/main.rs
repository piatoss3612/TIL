use std::fs::File; // File 구조체를 사용하기 위해 추가
use std::io::prelude::*; // read, write, flush 메서드를 사용하기 위해 추가
use std::net::{TcpListener, TcpStream}; // TcpListener, TcpStream 구조체를 사용하기 위해 추가

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap(); // TcpListener 객체 생성

    // listener.incoming() 메서드를 사용하여 연결 요청을 받음
    for stream in listener.incoming() {
        let stream = stream.unwrap(); // TcpStream 객체 가져옴

        handle_connection(stream); // 연결 요청 처리
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // 512 바이트 크기의 버퍼 생성

    stream.read(&mut buffer).unwrap(); // stream에서 읽어서 버퍼에 저장

    let get = b"GET / HTTP/1.1\r\n"; // GET 요청 확인

    // GET 요청이면 hello.html 파일을, 아니면 404.html 파일을 읽어서 응답
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let mut file = File::open(filename).unwrap(); // 파일 열기

    let mut contents = String::new(); // 파일 내용을 저장할 변수 생성
    file.read_to_string(&mut contents).unwrap(); // 파일 내용 읽어서 contents에 저장

    // status_line, contents를 사용하여 응답 생성
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap(); // 응답 전송
    stream.flush().unwrap(); // 응답 전송 완료
}
