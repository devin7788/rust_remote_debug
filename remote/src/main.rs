use std::net::TcpListener;
fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("0.0.0.0:9090").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
    }
}
