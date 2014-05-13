// Implements http://rosettacode.org/wiki/Hello_world/Web_server
#![cfg(not_tested)]

use std::io::net::tcp::{TcpListener, TcpStream};
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::io::{Acceptor, Listener};

fn handle_client(mut stream: TcpStream) {
    let response =
        bytes!("HTTP/1.1 200 OK\nContent-Type: text/html;\ncharset=UTF-8\n\n<doctype !html>\n<html>\n    <head>\n        <title>Bye-bye baby bye-bye</title>\n        <style>\n            body { background-color: #111 }\n            h1 { font-size:4cm; text-align: center; color: black; text-shadow: 0 0 2mm red}\n        </style>\n    </head>\n    <body>\n        <h1>Goodbye, world!</h1>\n    </body>\n</html>");
    match stream.write(response) {
        Ok(_) => println!("Response sent!"),
        Err(e) => println!("Failed sending response: {}!" , e)
    }
}

fn main() {
    let addr = SocketAddr{ip: Ipv4Addr(127, 0, 0, 1), port: 80,};
    let listener = TcpListener::bind(addr).unwrap();

    let mut acceptor = listener.listen();
    println!("Listening for connections on port {}" , addr . port);

    for stream in acceptor.incoming() {
        spawn(proc() { handle_client(stream.unwrap()); });
    }
}
