use std::net::{SocketAddr, TcpListener};
use std::time::{Duration, Instant};

use anyhow::Ok;
use mio::net::TcpStream;
use mio::{Events, Interest, Poll, Token};
const SERVER: Token = Token(0);

struct TCPServer {
    address: SocketAddr,
}

impl TCPServer {
    fn new(port: u32) -> Self {
        let address = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();

        TCPServer { address }
    }

    fn run(&mut self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.address).expect("Bind to port");

        let mut socket = TcpStream::connect(listener.local_addr()?)?;
        let mut poll = Poll::new().unwrap();

        poll.registry().register(
            &mut socket,
            Token(0),
            Interest::READABLE | Interest::WRITABLE,
        )?;

        let mut events = Events::with_capacity(1024);
        let start = Instant::now();
        let timeout = Duration::from_millis(500);

        loop {
            let elapsed = start.elapsed();

            if elapsed >= timeout {
                // Connection timed out
                return Ok(());
            }

            let remaining = timeout - elapsed;
            poll.poll(&mut events, Some(remaining))?;

            for event in &events {
                if event.token() == Token(0) {
                    // Something (probably) happened on the socket.
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}

fn main() {}
