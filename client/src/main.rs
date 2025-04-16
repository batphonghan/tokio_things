use std::{
    io,
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Instant,
};

use async_runtime::{executor::Executor, reciever::TcpReceiver, sender::TcpSender};
use data_layer::data::Data;
fn main() -> io::Result<()> {
    let mut executor = Executor::new();
    let mut handles = Vec::new();

    let start = Instant::now();

    for i in 0..4000 {
        let handle = executor.spawn(send_data(i, i as i16, format!("Hello server {}", i)));

        handles.push(handle);
    }

    std::thread::spawn(move || {
        loop {
            executor.poll();
        }
    });

    println!("Waiting for result ...");
    for handle in handles {
        match handle.recv().unwrap() {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        };
    }

    let duration = start.elapsed();

    println!("Time elapsed is expensive_functon: {:?}", duration);
    Ok(())
}

async fn send_data(field1: u32, field2: i16, field3: String) -> io::Result<String> {
    let stream = Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:7878")?));

    let message = Data {
        field1,
        field2,
        field3,
    };

    TcpSender {
        buffer: message.serialize()?,
        stream: stream.clone(),
    }
    .await?;

    let receiver = TcpReceiver {
        stream: stream.clone(),
        buffer: Vec::new(),
    };

    String::from_utf8(receiver.await?)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid UTF-8"))
}
