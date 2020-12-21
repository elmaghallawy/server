use std::io::prelude::*; // contains many traits that let us read from and write to streams
use std::net::{TcpListener, TcpStream};

fn main() {
    // listen to 127.0.0.1:7878 for incoming tcp streams
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
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
    // declare buffer (512 bytes) on the stack to hold data that's read in
    let mut buffer = [0; 512];
    // read the stream bytes and puts the data into the buffer
    stream.read(&mut buffer).unwrap();
    // convert bytes in the buffer to a string and print that string
    println!("Request: {}", String::from_utf8_lossy(&buffer[..])); // lossy replace invalid utf-8 squence

    // send response
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    // convert string data to bytes and
    // write takes those bytes and send them directly down the connection
    stream.write(response.as_bytes()).unwrap();
    // flush will wait and prevent the program from continuing
    // until all the bytes are written into the connection
    stream.flush().unwrap();
}
