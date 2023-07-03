#![allow(unused_imports, unused_variables)]
// include the generated source code in this file.
pub mod helloservice {
    include!("helloservice.rs");
}

#[derive(Default)]
pub struct HelloServiceImpl {}
use std::net::SocketAddr;

use helloservice::{my_service_server::{MyService, MyServiceServer}, Hello, HelloResponse};
use tonic::{Response, Request};

#[tonic::async_trait]
impl MyService for HelloServiceImpl {
    async fn say_hello(&self,request:tonic::Request<Hello>,) -> std::result::Result<tonic::Response<HelloResponse>,tonic::Status> {
        Ok(Response::new(HelloResponse {
            sentence: format!("Hello, {}", request.get_ref().name)
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hello = helloservice::Hello {
        name: "kumarmo2".to_string(),
    };
    let addr: SocketAddr = "127.0.0.1".parse().unwrap();
    tonic::transport::Server::builder().add_service(MyServiceServer::new(HelloServiceImpl {})).serve(addr).await?;
    Ok(())
}
