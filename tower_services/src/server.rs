use std::{
    error::Error,
    fmt::{Debug, Display},
    future::Future,
    net::TcpStream,
    pin::Pin,
    str::FromStr,
    time::Duration,
};

use anyhow::bail;
use tokio::time::{Sleep, error::Elapsed};
use tower::Service;

pub struct Server {}

pub struct Request {}
pub struct Response {}

#[derive(Debug, Clone)]
struct Timeout<S> {
    inner: S,
    timeout: Duration,
}

impl<S> Timeout<S> {
    pub fn new(inner: S, timeout: Duration) -> Self {
        Self { inner, timeout }
    }
}

pub struct ResponseFuture<F> {
    response_future: F,
    sleep: Sleep,
}

// impl<F, Response, Error> Future for ResponseFuture<F>
// where
//     F: Future<Output = Result<Response, Error>>,
// {
//     type Output = Result<Response, Error>;
//     fn poll(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         let f = Box::pin(self.response_future);
//         match f.poll(cx) {}
//         todo!()
//     }
// }

#[derive(Debug)]
enum MyErr<T> {
    InnerError(T),
    Timeout,
}

impl<T: Debug> Error for MyErr<T> {}

impl<T> Display for MyErr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "ERROR")
    }
}

impl<S, Request: 'static> Service<Request> for Timeout<S>
where
    S: Service<Request> + Clone + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<S::Response, MyErr<S::Error>>>>>;
    type Response = S::Response;
    type Error = MyErr<S::Error>;

    fn call(&mut self, request: Request) -> Self::Future {
        // let response_future = self.inner.call(req);

        // Get an owned clone of `&mut self`
        let mut this = self.clone();

        let sleep = tokio::time::sleep(this.timeout);

        // ResponseFuture {
        //     response_future,
        //     sleep,
        // }
        // Box::pin(async move { response_future.await })

        Box::pin(async move {
            let result = tokio::time::timeout(this.timeout, this.inner.call(request)).await;

            match result {
                Ok(Ok(response)) => Ok(response),
                Ok(Err(error)) => Err(MyErr::InnerError(error)),
                Err(timeout) => Err(MyErr::Timeout),
            }
        })
    }

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.inner
            .poll_ready(cx)
            .map_err(|err| MyErr::InnerError(err))
    }
}

// impl Server {
//     async fn run<F>(self, handler: F) -> Result<(), Error>
//     where
//         F: Fn(Request) -> Response + Clone + Send + 'static,
//     {
//         loop {
//             let handler = handler.clone();
//             tokio::spawn(async move {
//                 handler(Request {});
//             });
//         }
//         Ok(())
//     }
// }

pub trait Container {
    async fn item(&self) -> String;
}
