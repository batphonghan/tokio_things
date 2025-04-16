use bytes::Bytes;
use core::fmt;
use http_body_util::Full;
use hyper::rt::Read;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Error, Request, Response, body};
use hyper_util::{rt::TokioIo, service::TowerToHyperService};
use log::Log;
use serde::ser;
use std::convert::Infallible;
use std::fmt::{Display, write};
use std::future::{self, Future, Pending, Ready, ready};
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::pin::Pin;
use std::process::Output;
use std::task::Poll;
use std::time::{Duration, Instant};
use timeout::timeout::TimeOut;
use tokio::net::TcpListener;
use tokio::time::{Sleep, sleep, timeout};
use tower::ServiceBuilder;

#[path = "../middleware/mod.rs"]
mod logging;

#[path = "../middleware/mod.rs"]
mod timeout;

use logging::logging::{BoxFuture, Logging, LoggingFuture};
use pin_project::pin_project;
async fn hello(_: Request<body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

// #[path = "../bin/server.rs"]
// mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr: SocketAddr = ([127, 0, 0, 1], 3456).into();

    let listener = TcpListener::bind(addr).await?;
    env_logger::init();
    loop {
        let (mut stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);
        let service = HelloService {};
        let service = Logging::new(service);
        let service = TimeOut::new(service, 1);
        // hyper::service::service_fn(f)

        let svc = TowerToHyperService::new(service);

        let svc = ServiceBuilder::new().service(svc);

        if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
            log::error!("{:?}", err);
        }
    }

    Ok(())
}

#[derive(Clone)]
struct HelloService;

impl tower::Service<Request<body::Incoming>> for HelloService {
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Response<Full<Bytes>>, Infallible>>;
    type Response = Response<Full<Bytes>>;
    fn call(&mut self, req: Request<body::Incoming>) -> Self::Future {
        Box::pin(async move {
            sleep(Duration::from_millis(900)).await;
            let b = Full::<Bytes>::from("Hello World");
            let resp = Response::new(b);
            (Ok(resp))
        })
    }

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
}
