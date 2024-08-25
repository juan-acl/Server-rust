use std::net::TcpListener;
fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8080";
    let end_point: String = HOST.to_owned() + ":" + PORT;
    let listener = TcpListener::bind(end_point).unwrap();
    println!("Server is listening on {}", PORT);
    for stream in listener.incoming() {
        stream.unwrap();
        println!("Connection established!");
    }

    println!("Hello, world!");
}
