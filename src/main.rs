extern crate tokio;

use std::net::SocketAddr;
use tokio::io;
use tokio::net::{TcpListener};
use tokio::prelude::*;

struct Server {
    listening_socket: SocketAddr,
}

impl Server {

    pub fn new() -> Self {
        Server {
            listening_socket: "127.0.0.1:6969".parse().unwrap(),
        }
    }

    pub fn start(&self) {
		let listener = TcpListener::bind(&self.listening_socket).unwrap();

        let server = listener.incoming().for_each(|socket| {
            println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());
            let buffer = vec![];
            let future = io::read_to_end(socket, buffer);
            let ()  = future.poll();
            println!("reading {:?}", buffer);

            let connection = io::write_all(socket, "hello world\n")
                .then(|res| {
                    println!("write message; success={:?}", res.is_ok());
                    Ok(())
                });

            tokio::spawn(connection);

            Ok(())
        })
        .map_err(|err| {
            println!("accept error = {:?}", err);

        });

		println!("Listening on {}", self.listening_socket);

        tokio::run(server);
    }
}



fn main() {
    Server::new().start();
}
