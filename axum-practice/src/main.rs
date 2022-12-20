use axum::{Router, routing::get};


#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new().route("/", get(|| async { "hello, world" }));


    axum::Server::bind(&"120.0.0.1:3000".parse().unwrap())
    .serve(app.into_make_service())
        .await
    .unwrap(); 
}
