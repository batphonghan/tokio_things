use std::{panic::catch_unwind, pin::Pin, time::Instant};

use tokio::io::AsyncReadExt;

#[path = "../middleware/mod.rs"]
mod slow_read;

#[tokio::main]

async fn main() -> Result<(), tokio::io::Error> {
    let result = catch_unwind(|| {
        println!("Insile catch");
        panic!("Not ok")
    });

    println!("Cach that{}", result.is_ok());

    // return Ok(());
    let mut buf = vec![0u8; 128 * 1024];
    let mut reader = tokio::fs::File::open("/dev/urandom").await?;
    let mut reader = slow_read::slow_read::SlowRead::new(reader);
    let before = Instant::now();

    let mut reader = unsafe { Pin::new_unchecked(&mut reader) };
    reader.read_exact(&mut buf).await?;

    println!("Read {} in {:?}", buf.len(), before.elapsed());
    Ok(())
}
