use std::{pin::Pin, task::Poll, time::Duration};

use futures::FutureExt;
use pin_project::pin_project;
use tokio::{
    io::AsyncRead,
    time::{Instant, Sleep, sleep},
};

pub struct SlowRead<R> {
    reader: R,

    sleep: Sleep,
}

impl<R> SlowRead<R> {
    pub fn new(reader: R) -> Self {
        SlowRead {
            reader,
            sleep: sleep(Duration::from_secs(6)),
        }
    }
}

impl<R> AsyncRead for SlowRead<R>
where
    R: AsyncRead + Unpin,
{
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let (mut sleep, mut reader) = unsafe {
            let mut this = self.get_unchecked_mut();
            (
                Pin::new_unchecked(&mut this.sleep),
                Pin::new_unchecked(&mut this.reader),
            )
        };

        match sleep.as_mut().poll(cx) {
            std::task::Poll::Pending => Poll::Pending,
            Poll::Ready(_) => {
                println!("Reseting");
                sleep.reset(Instant::now() + Duration::from_millis(100));

                Pin::new(&mut reader).poll_read(cx, buf)
            }
        }
    }
}
