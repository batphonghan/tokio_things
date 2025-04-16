use std::{net::UdpSocket, thread};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Bind to 8888");

    loop {
        let mut buf = [0; 1500];
        let socket = socket.try_clone().expect("clone socket");

        println!("Start serce: ");
        match socket.recv_from(&mut buf) {
            Ok((bytes_read, addr)) => {
                thread::spawn(move || {
                    println!("Halle connect from {:?}", addr);
                    socket.send_to(&buf, addr).expect("sent a respon back");
                });
            }
            Err(e) => {
                println!("Could not receive a datagram : {}", e);
            }
        }
    }
}
