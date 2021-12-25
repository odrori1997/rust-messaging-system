use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

const LOCAL: &str = "127.0.0.1:7000";
const MSG_SIZE: usize = 32;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main(){
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("Unable to set non-blocking");

    let (tx, rx) = mpsc::channel::<String>(); // create a String channel

    thread::spawn(move || loop { // first, create a thread that reads from server
        let mut buff = vec![0; MSG_SIZE];

        // read from server
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("Message received: {:?}", msg); // print message, in bytes
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server terminated");
                break;
            }
        }

        // now, read string from channel receiver and forward to client
        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0); // why do we need this?
                client.write_all(&buff).expect("Writing to socket failed. ");
                println!("Message sent {:?}", msg);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }
        sleep();
    });

    println!("Write a message: ");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Unable to read from stdin");
        let msg = buff.trim().to_string();
        if msg ==  ":quit" || tx.send(msg).is_err() {break}
    }

    println!("Exiting...")
}