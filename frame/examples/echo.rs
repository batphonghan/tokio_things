use core::panic;
use std::collections::HashMap;
use std::io::Cursor;
use std::os::unix::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Error, Ok, Result};
use bytes::{Buf, Bytes, BytesMut};
use mini_redis::Command::{self, Get, Set};
use mini_redis::Frame;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
// use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("localhost:8080").await?;

    let (tx, _rx) = broadcast::channel::<(std::net::SocketAddr, String)>(100);
    loop {
        let (mut socket, addr) = listener.accept().await?;

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);

            let mut line = String::new();

            loop {
                tokio::select! {
                    bytes_read = reader.read_line(&mut line) => {
                        let bytes_read = bytes_read?;
                        if bytes_read == 0 {
                            break;
                        }

                        tx.send((addr, line.clone()))?;
                    }
                    rs = rx.recv() => {
                        let (_addr, msg) = rs?;
                        if addr != _addr {
                            writer.write_all(msg.as_bytes()).await?;
                            line.clear();
                        }
                    }
                }
            }

            Ok(())
        });
    }

    Ok(())
}
