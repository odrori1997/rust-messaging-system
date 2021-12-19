extern crate mio;
use mio::*;
use std::net::SocketAddr;
use mio::net::*;
use std::collections::HashMap;

const SERVER_TOKEN: Token = Token(0);
const MAX_SOCKETS: usize = 2048;

fn main() {
    let mut event_loop = Poll::new().unwrap();

    let address = "0.0.0.0:1000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    event_loop.register(&server_socket, 
                        Token(0),
                        Ready::readable(),
                        PollOpt::edge()).unwrap();

    let mut sockets = HashMap::new();
    let mut next_socket_index = 0;
    let mut events = Events::with_capacity(1024);

    event_loop.poll(&mut events, None).unwrap();

    for event in &events {
        match event.token() {
            SERVER_TOKEN => {
                loop {
                    let mut buf = [0; 2048];
                    match server_socket.accept() {
                        Err(e) => {
                            println!("Error reading socket: {}", e);
                            return
                        },
                        Ok((socket, _)) => {
                            if next_socket_index == MAX_SOCKETS {
                                return;
                            }
                            let token = Token(next_socket_index);
                            next_socket_index += 1;
                            event_loop.register(&socket,
                                        token,
                                        Ready::readable(),
                                        PollOpt::edge()).unwrap();
                            sockets.insert(token, socket);
                        },

                    }
                }
            }
            token => {
                
            }
        }
    }

}
