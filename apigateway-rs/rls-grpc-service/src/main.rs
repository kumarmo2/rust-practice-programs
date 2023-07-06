#![allow(unused_variables)]
pub(crate) mod envoy;

pub(crate) mod validate;

pub(crate) mod xds;

pub(crate) mod udpa;

use envoy::{
    extensions::common::ratelimit::v3::rate_limit_descriptor::Entry,
    service::ratelimit::v3::{
        rate_limit_service_server::{self, RateLimitServiceServer},
        RateLimitRequest, RateLimitResponse,
    },
};
use tonic::{transport::Server, Code, Response};
struct RateLimitServiceImpl {}

#[tonic::async_trait]
impl rate_limit_service_server::RateLimitService for RateLimitServiceImpl {
    async fn should_rate_limit(
        &self,
        request: tonic::Request<RateLimitRequest>,
    ) -> std::result::Result<Response<RateLimitResponse>, tonic::Status> {
        println!("here");
        let request = request.into_inner();
        for descriptor in request.descriptors.iter() {
            for Entry { key, value } in descriptor.entries.iter() {
                println!("key: {}, value: {}", key, value);
            }
        }
        Ok(Response::new(RateLimitResponse {
            overall_code: Code::Ok as i32,
            statuses: vec![],
            quota: None,
            response_headers_to_add: vec![],
            request_headers_to_add: vec![],
            raw_body: vec![],
            dynamic_metadata: None,
        }))
        // Err(tonic::Status::new(Code::Ok, "not passed"))
        // todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let x = envoy::service::ratelimit::v3::RateLimitRequest {};
    Server::builder()
        .add_service(RateLimitServiceServer::new(RateLimitServiceImpl {}))
        .serve("0.0.0.0:9000".parse().unwrap())
        .await?;
    Ok(())
}
