use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::fs;
use std::thread::{self, Thread};
use std::time::Duration;
use serverTrial::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);



    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        pool.execute ( || {handle_connection(stream)});

        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) -> () {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();



    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (staus, fname ) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK", "index.html")
    }
    else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    

    let status_line = staus;
    let contents = fs::read_to_string(fname).unwrap();

    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();


}