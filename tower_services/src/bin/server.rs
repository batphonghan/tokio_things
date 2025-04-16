use std::{collections::HashMap, sync::Arc};

use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

use crate::Connection as Conn;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("Accepted");
        let db = db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    // let (r, mut w) = socket.split();
    let mut connect = Connection::new(socket);
    let mut count = 0;
    while let Some(frame) = connect.read_frame().await.unwrap() {
        count += 1;
        println!("Count {count}");
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                let mut db = db.lock().await;
                db.insert(cmd.key().to_string(), cmd.value().clone());

                println!("Set {:?}", cmd.key().to_string());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                let db = db.lock().await;
                if let Some(value) = db.get(cmd.key()) {
                    println!("Get {:?}", value.clone());
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => {
                println!(">>>>>>>>>> {:?}", cmd);
                Frame::Null
            }
        };

        connect.write_frame(&response).await.unwrap();
    }
    println!("END");
}
