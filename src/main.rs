use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use hello::ThreadPool;

fn main (){
    println!("hello world");
    let listenser = TcpListener::bind("127.0.0.1:8899").unwrap();

    let pool = ThreadPool::new(4);
    for stream in listenser.incoming().take(2) {
        println!("hhh listen ");
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
        /*thread::spawn(|| {
            handle_connection(stream);
        });*/
         
    }
}

fn handle_connection (mut stream:TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer));
   
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status, path) =  if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_millis(10000));
        ("HTTP/1.1 200 OK\r\n", "hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n", "404.html")
    };
   
    let contents = fs::read_to_string(path).unwrap();
    let response = format!(
        "{}Content-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}