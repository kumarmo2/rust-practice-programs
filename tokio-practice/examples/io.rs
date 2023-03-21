use tokio::{fs::File, io::{AsyncWriteExt, self, AsyncReadExt}};


const FILE_PATH: &str = "/home/manya/code/rust/rust-practice-programs/tokio-practice/random.txt";

#[tokio::main]
async fn main() {
    // write_example().await;
    read_to_end_example().await;
}


async fn read_to_end_example() {
    let mut file = File::open("/home/manya/.bashrc").await.unwrap();
    let mut buf = Vec::new();

    file.read_to_end(&mut buf).await.unwrap();
    println!("file read, total bytes read: {}", buf.len());

}

async fn write_example() {

    let mut file = File::create(FILE_PATH).await.unwrap();

    let n = file.write(b"kumarmo2").await.unwrap();

    println!("num of bytes written: {}", n);

}
