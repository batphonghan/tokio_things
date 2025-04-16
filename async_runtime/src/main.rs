use std::task::Poll;

use async_runtime::executor;

pub struct CountingFuture {
    count: i32,
}

impl Future for CountingFuture {
    type Output = ();
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.count == 4 {
            println!("Counting is done");
            Poll::Ready(())
        } else {
            println!("Counting is not done {}", self.count);
            self.count += 1;

            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
fn main() {
    let counter = CountingFuture { count: 0 };

    let counter2 = CountingFuture { count: 0 };

    let mut executor = executor::Executor::new();

    let handle = executor.spawn(counter);
    let _handle = executor.spawn(counter2);

    std::thread::spawn(move || {
        loop {
            executor.poll();
        }
    });

    let result = handle.recv().unwrap();
    let result = _handle.recv().unwrap();

    println!("Result: {:?}", result);
}
