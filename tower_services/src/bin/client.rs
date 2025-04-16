use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    // The `Sender` handles are moved into the tasks. As there are two
    // tasks, we need a second `Sender`.
    let tx2 = tx.clone();

    // Spawn two tasks, one gets a key, the other sets a key
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            responder: resp_tx,
        };

        tx.send(cmd).await.unwrap();

        let resp = resp_rx.await;
        println!("Got = {:?}", resp)
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            responder: resp_tx,
        };

        tx2.send(cmd).await.unwrap();

        // await respose
        let resp = resp_rx.await;
        println!("Got = {:?}", resp)
    });

    let mut client = client::connect("127.0.0.1:6379").await.unwrap();
    // The `move` keyword is used to **move** ownership of `rx` into the task.
    let manager = tokio::spawn(async move {
        // Establish a connection to the server

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, responder } => {
                    let resp = client.get(&key).await;
                    let _ = responder.send(resp);
                }
                Set {
                    key,
                    val,
                    responder,
                } => {
                    let resp = client.set(&key, val).await;

                    let _ = responder.send(resp);
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        responder: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        responder: Responder<()>,
    },
}
