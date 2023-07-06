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
use tonic::{transport::Server, Response};

use crate::envoy::{
    config::core::v3::HeaderValue, service::ratelimit::v3::rate_limit_response::Code,
};
struct RateLimitServiceImpl {}

#[tonic::async_trait]
impl rate_limit_service_server::RateLimitService for RateLimitServiceImpl {
    async fn should_rate_limit(
        &self,
        request: tonic::Request<RateLimitRequest>,
    ) -> std::result::Result<Response<RateLimitResponse>, tonic::Status> {
        // https://www.envoyproxy.io/docs/envoy/latest/api-v3/service/ratelimit/v3/rls.proto.html#service-ratelimit-v3-ratelimitresponse
        println!("here");
        let request = request.into_inner();
        for descriptor in request.descriptors.iter() {
            for Entry { key, value } in descriptor.entries.iter() {
                println!("key: {}, value: {}", key, value);
            }
        }
        let ok = rand::random::<bool>();
        println!("ok: {}", ok);
        match ok {
            true => Ok(Response::new(RateLimitResponse {
                overall_code: Code::Ok as i32,
                statuses: vec![],
                quota: None,
                response_headers_to_add: vec![],
                request_headers_to_add: vec![],
                raw_body: vec![],
                dynamic_metadata: None,
            })),
            false => Ok(Response::new(RateLimitResponse {
                overall_code: Code::OverLimit as i32,
                statuses: vec![],
                response_headers_to_add: vec![],
                request_headers_to_add: vec![],
                raw_body: vec![],
                dynamic_metadata: None,
                quota: None,
            })),
            // false => Err(tonic::Status::),
        }
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
