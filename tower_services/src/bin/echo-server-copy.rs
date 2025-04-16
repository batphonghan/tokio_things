use std::time::Duration;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> io::Result<()> {
    tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:6142").await?;
        while let Ok(mut socket) = listener.accept().await {
            let (mut r, mut w) = socket.0.split();

            tokio::io::copy(&mut r, &mut w).await?;

            // drop(r);
            // println!("Shuting down");
            // drop(w);
        }

        // loop {
        //     let (mut socket, _) = listener.accept().await?;

        //     tokio::spawn(async move {
        //         let mut buf = vec![0; 1024];

        //         loop {
        //             match socket.read(&mut buf).await {
        //                 // Return value of `Ok(0)` signifies that the remote has
        //                 // closed
        //                 Ok(0) => return,
        //                 Ok(n) => {
        //                     // Copy the data back to socket
        //                     if socket.write_all(&buf[..n]).await.is_err() {
        //                         // Unexpected socket error. There isn't much we can
        //                         // do here so just stop processing.
        //                         return;
        //                     }
        //                 }
        //                 Err(_) => {
        //                     // Unexpected socket error. There isn't much we can do
        //                     // here so just stop processing.
        //                     return;
        //                 }
        //             }
        //         }
        //     });
        // }

        Ok::<_, io::Error>(())
    });

    let socket = TcpStream::connect("127.0.0.1:6142").await?;

    let (mut rd, mut wt) = io::split(socket);

    tokio::spawn(async move {
        wt.write_all(b"hi\r\n").await?;
        wt.write_all(b"hi 2\r\n").await?;
        wt.write_all(b"hi 4\r\n").await?;
        wt.write_all(b"hi 4\r\n").await?;
        wt.write_all(b"hi 5\r\n").await?;

        // wt.flush().await?;
        wt.shutdown().await?;
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        println!("Start read");
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        println!("Got {:?}", String::from_utf8(buf[..n].to_vec()).unwrap());
    }

    Ok(())
}
