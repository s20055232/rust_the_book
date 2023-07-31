use simple_web::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));

        println!("連線建立！");
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    // 這邊會收到兩次Request，一次是預期中的 GET / HTTP/1.1
    // 一次是 GET /favicon.ico HTTP/1.1
    // 而如果我把 41-47行解註解會出現錯誤，因為我在那邊再次等候stream傳輸資料，而第一次的資料已被使用，所以我print不出東西
    // 只能print出第二次的Request
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();

    // 下面解開註解，可以查看http request長什麼樣子
    // let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("請求：{:#?}", http_request);

    // let status_line = "HTTP/1.1 200 OK";
    // let contents = fs::read_to_string("hello.html").unwrap();
    // let length = contents.len();
    // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // stream.write_all(response.as_bytes()).unwrap();
}
