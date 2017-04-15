use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};

fn main(){
    //bind allows us to create a connection on the port
    //and gets it ready to accept connections.
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    let stream = listener.accept().unwrap().0;
    read_request(stream);
}

fn read_request(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream);

    for line in reader.by_ref().lines(){
        if line.unwrap() == "" {
            break;
        }
    }

    send_response(reader.into_inner());
}

fn send_response(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\n\n<html><body>hello, World!</body></html>";

    stream.write_all(response.as_bytes()).unwrap();
}
