use std::{task::Poll, time::Instant};

pub struct Sleep {
    when: Instant,
}

impl Sleep {
    pub fn new(duration: std::time::Duration) -> Self {
        Sleep {
            when: Instant::now() + duration,
        }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let now = Instant::now();
        if now < self.when {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}
