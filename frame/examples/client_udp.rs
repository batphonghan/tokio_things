use std::{
    env,
    io::stdin,
    net::{IpAddr, SocketAddr, UdpSocket},
    thread,
};

use ipnetwork::IpNetwork;

fn main() {

    // let socket = UdpSocket::bind("0.0.0.0:8889").expect("Bind to 8889");

    // socket.connect("0.0.0.0:8888").expect("Connect to 8888");

    // loop {
    //     let mut buf = String::new();

    //     let bytes_read = stdin().read_line(&mut buf).expect("read line");

    //     socket
    //         .send(buf[..bytes_read].as_bytes())
    //         .expect("sent back to server");

    //     let mut buf = [0; 1500];
    //     match socket.recv_from(&mut buf) {
    //         Ok((b, addr)) => {
    //             let s = std::str::from_utf8(&buf[..b]).expect("parse utf 8 string back");
    //             println!("Got back {} from {}", s, addr.to_string());
    //         }
    //         Err(e) => eprintln!("Error read back {}", e),
    //     };
    // }
}
