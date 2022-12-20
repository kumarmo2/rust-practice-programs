use mini_redis::{Connection, Frame, Command};
use tokio::net::{TcpListener, TcpStream};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};


type Db = Arc<Mutex<HashMap<String, Bytes>>>;


#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = Arc::clone(&db);
        process(socket, db).await;
    }

}



async fn process(socket: TcpStream, db: Db) {

    // the `COnnection` lets us read/write redis **frames** instead of
    // byte steams. The `Connection` type is defined by mini-redis.

    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Command::Get(cmd) => {

                let key = cmd.key();
                let lock_guard = db.lock().unwrap();
                if let Some(val) = lock_guard.get(key) {
                    Frame::Bulk(val.clone())
                } else {
                  Frame::Null

                }
            },
            Command::Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            _ => todo!()
        };
        connection.write_frame(&response).await.unwrap();
    }
    println!("processing ended");
}


