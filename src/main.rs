use std::net::TcpListener;

fn main() {
    // listen to 127.0.0.1:7878 for incoming tcp streams
    // print "Connection established!" when it gets a stream
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => println!("connection established"),
            Err(e) => println!("Opss error {}", e),
        }
    }
}
