use core::fmt;
use std::{
    pin::{Pin, pin},
    task::Poll,
    time::Duration,
};

use futures::FutureExt;
use hyper::Request;
use pin_project::pin_project;
use tokio::time::{Sleep, sleep};

#[derive(Clone)]
pub struct TimeOut<S> {
    inner: S,
    secs: u64,
}

impl<S> TimeOut<S> {
    pub fn new(inner: S, secs: u64) -> Self {
        TimeOut { inner, secs }
    }
}

impl<S, B> tower::Service<Request<B>> for TimeOut<S>
where
    S: tower::Service<Request<B>> + Clone + Send + 'static,
    S::Future: Send,
    B: Send + 'static,
{
    type Error = TimeoutError<S::Error>;
    type Future = TimeoutFuture<S::Future>;
    type Response = S::Response;

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let mut inner = self.inner.clone();
        let secs = self.secs;

        let inner_fut = inner.call(req);

        let t = TimeoutFuture::new(inner_fut, secs);
        t
    }

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner
            .poll_ready(cx)
            .map_err(|e| TimeoutError::ServiceErr(e))
    }
}

#[pin_project]
pub struct TimeoutFuture<Service> {
    #[pin]
    pub inner: Service,

    // #[pin]
    pub timeout: Pin<Box<Sleep>>,
}

impl<S> TimeoutFuture<S> {
    fn new(inner: S, secs: u64) -> Self {
        let timeout = Box::pin(sleep(Duration::from_secs(secs)));
        TimeoutFuture { inner, timeout }
    }
}

impl<S, Response, Error> Future for TimeoutFuture<S>
where
    S: Future<Output = Result<Response, Error>>,
{
    type Output = Result<Response, TimeoutError<Error>>;
    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // self.timeout.poll_unpin(cx);

        // todo!()
        let mut this = self.project();

        let timeout_fut = Pin::new(&mut this.timeout);

        match timeout_fut.poll(cx) {
            std::task::Poll::Pending => {}
            std::task::Poll::Ready(_) => return Poll::Ready(Err(TimeoutError::Timeout)),
        };

        let inner_fut = this.inner;

        match inner_fut.poll(cx) {
            std::task::Poll::Pending => Poll::Pending,
            std::task::Poll::Ready(v) => Poll::Ready(v.map_err(|e| TimeoutError::ServiceErr(e))),
        }
    }
}

impl<E> fmt::Display for TimeoutError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("request timed out")
    }
}

impl<T> std::error::Error for TimeoutError<T> where T: fmt::Debug {}

#[derive(Debug, Clone)]
pub enum TimeoutError<E> {
    Timeout,
    ServiceErr(E),
}
