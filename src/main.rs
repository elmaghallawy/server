use std::fs;
use std::io::prelude::*; // contains many traits that let us read from and write to streams
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    // listen to 127.0.0.1:7878 for incoming tcp streams
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => println!("Opss error {}", e),
        }
    }
}

// TcpStream instance keeps track of what data it returns to us internally
// it might read more data than we asked for and save that data for the next
// time we ask for data therefore we made it mutable because its internal state
// might change
fn handle_connection(mut stream: TcpStream) {
    // PARSE REQUEST

    // declare buffer (512 bytes) on the stack to hold data that's read in
    let mut buffer = [0; 512];
    // read the stream bytes and puts the data into the buffer
    stream.read(&mut buffer).unwrap();
    // convert bytes in the buffer to a string and print that string
    println!("Request: {}", String::from_utf8_lossy(&buffer[..])); // lossy replace invalid utf-8 squence

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // SEND RESPONSE
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    // convert string data to bytes and
    // write takes those bytes and send them directly down the connection
    stream.write(response.as_bytes()).unwrap();
    // flush will wait and prevent the program from continuing
    // until all the bytes are written into the connection
    stream.flush().unwrap();
}
