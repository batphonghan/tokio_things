use std::{
    error::Error,
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use anyhow::Context;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Bind to 8888");

    loop {
        for (stream, _) in listener.accept() {
            thread::spawn(move || handle_client(stream).expect("hanle connect"));
        }
    }
}

fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {
    println!("Incomming connection from: {:?}", stream.peer_addr());

    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;

        if bytes_read == 0 {
            println!("Connection closed");
            return Ok(());
        }
        let choices = [1, 2, 3, 4];
        let mut rng = thread_rng();
        let c = choices.choose(&mut rng).expect("choose one");

        let dur = Duration::from_secs(*c);
        std::thread::sleep(dur);

        stream.write_all(&buf[..bytes_read])?;
    }
}
