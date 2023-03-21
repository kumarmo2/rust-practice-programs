use std::fs::read;

use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};


#[tokio::main]
async fn main() {

    let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();

    loop {

    let (stream, _ ) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(stream).await;
        });

    }

}



async fn process(mut stream: TcpStream) {

    let mut buf = [0, 2];

    loop {
        match stream.read(&mut buf).await {
            Ok(0) => return,
            Ok(n) => {
                println!("read {} bytes", n);
                stream.write_all(&buf).await.unwrap();
            },
            Err(e) => println!("err: {:?}", e) ,
        }
    }
}
