use std::{
    io::{self, Read},
    net::TcpStream,
    ops::Mul,
    sync::{Arc, Mutex},
    task::Poll,
    time::{Duration, Instant},
};

pub struct TcpReceiver {
    pub stream: Arc<Mutex<TcpStream>>,
    pub buffer: Vec<u8>,
}

impl Future for TcpReceiver {
    type Output = io::Result<Vec<u8>>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut stream = match self.stream.try_lock() {
            Ok(stream) => stream,
            Err(_) => {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        stream.set_nonblocking(true)?;

        let mut local_buf = [0; 1024];

        match stream.read(&mut local_buf) {
            Ok(0) => Poll::Ready(Ok(self.buffer.to_vec())),
            Ok(n) => {
                std::mem::drop(stream);
                self.buffer.extend_from_slice(&local_buf[..n]);
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err((e)) => Poll::Ready(Err(e)),
        }
    }
}
