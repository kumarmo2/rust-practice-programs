
use std::collections::HashMap;
use std::str::FromStr;
use tokio::task;
use tokio::sync::{mpsc, oneshot};
use bytes::Bytes;
use futures::future;



#[derive(Debug)]
struct GetRequest<'key> {
    key: &'key str,
    sender: oneshot::Sender<Option<Bytes>>
}

#[derive(Debug)]
struct SetRequest {
    key: String,
    value: Bytes,
    sender: oneshot::Sender<()>
}

#[derive(Debug)]
enum Request<'key> {
    Get(GetRequest<'key>),
    Set(SetRequest)
}


#[tokio::main]
async fn main() {
    let mut db = HashMap::<String, Bytes>::new();

    let (tx, mut rx) =  mpsc::channel::<Request>(5);


    let _ = task::spawn(async move {
        while let Some(request) = rx.recv().await {
            match request {
                Request::Get(request) => {
                    request.sender.send(db.get(request.key).map(|val| { Bytes::clone(val)})).unwrap();
                },
                Request::Set(request) => {
                    db.insert(request.key, request.value);
                    request.sender.send(()).unwrap();
                },
            };
        }
    });

    let key_str = "name";
    let key = String::from_str(key_str).unwrap();

    let request_sender = tx.clone();

    let h1 = task::spawn(async move {
        let (tx, rx) = oneshot::channel();
        let request = SetRequest { key, value: "Kumarmo2".into(), sender: tx};
        request_sender.send(Request::Set(request)).await.unwrap();
        rx.await.unwrap()
    });


    let request_sender = tx.clone();
    let h2 = task::spawn(async move {
        let (tx, rx) = oneshot::channel();
        let request = GetRequest { key: key_str, sender: tx};
        request_sender.send(Request::Get(request)).await.unwrap();
        println!("get: {:?}", rx.await.unwrap());
    });

    future::join(h1, h2).await;

}
