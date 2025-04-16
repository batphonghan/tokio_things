use futures::future::FutureExt;
use hyper::Request;
use pin_project::pin_project;
use std::{pin::Pin, time::Instant};
#[pin_project]
pub struct LoggingFuture<S> {
    #[pin]
    pub inner: S,
    pub start: Instant,
    pub method: String,
    pub path: String,
}

impl<S> Future for LoggingFuture<S>
where
    S: Future,
{
    type Output = S::Output;
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        log::info!("THIS IS START POOL");
        let this = self.project();

        let resp = match this.inner.poll(cx) {
            std::task::Poll::Pending => {
                log::debug!("Pending");
                return std::task::Poll::Pending;
            }
            std::task::Poll::Ready(resp) => {
                log::debug!("Ready");
                resp
            }
        };

        log::info!("End {:?} {:?}", this.method, this.path);
        log::info!("THIS IS END POOL TOOK {:?}", this.start.elapsed());

        std::task::Poll::Ready(resp)
        // this.inner.poll(cx)
    }
}

#[derive(Debug, Clone)]
pub struct Logging<S> {
    pub inner: S,
}

impl<S> Logging<S> {
    pub fn new(s: S) -> Self {
        Logging { inner: s }
    }
}

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

impl<S, B> tower::Service<Request<B>> for Logging<S>
where
    S: tower::Service<Request<B>> + Send + Clone + 'static,
    S::Future: Send,
    B: Send + 'static,
{
    type Error = S::Error;
    type Future = LoggingFuture<S::Future>;
    type Response = S::Response;
    fn call(&mut self, req: Request<B>) -> Self::Future {
        let mut inner = self.inner.clone();
        let method = req.method().clone();
        let uri = req.uri().clone();
        log::debug!("Start tracce {} {}", method, uri.path());

        let inner = inner.call(req);
        let ft = LoggingFuture {
            inner,
            start: Instant::now(),
            method: method.to_string(),
            path: uri.path().to_string(),
        };

        ft
    }

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
}
