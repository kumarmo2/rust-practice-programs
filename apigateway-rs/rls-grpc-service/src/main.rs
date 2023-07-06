#![allow(unused_variables)]
pub(crate) mod envoy;

pub(crate) mod validate;

pub(crate) mod xds;

pub(crate) mod udpa;

#[cfg(test)]
mod test;
pub(crate) mod throttler;

use std::hash::Hash;

use envoy::{
    extensions::common::ratelimit::v3::{rate_limit_descriptor::Entry, RateLimitDescriptor},
    service::ratelimit::v3::{
        rate_limit_service_server::{self, RateLimitServiceServer},
        RateLimitRequest, RateLimitResponse,
    },
};
use redis::RedisResult;
use serde::{Deserialize, Serialize};
use tonic::{transport::Server, Response};

use crate::envoy::service::ratelimit::v3::rate_limit_response::Code;

struct RateLimitServiceImpl {
    // TODO: use some sort of connection pooling.
    redis_client: redis::Client,
    rate_limit_configs: Vec<RateLimitConfig>,
}

#[derive(Serialize, Deserialize, Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl TryFrom<&str> for HttpMethod {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "get" => Ok(HttpMethod::Get),
            "post" => Ok(HttpMethod::Post),
            "put" => Ok(HttpMethod::Put),
            "delete" => Ok(HttpMethod::Delete),
            _ => Err("unsupported httpmethod"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum TimeUnit {
    S = 1,
    M = 60,
    H = 3600,
    D = 86400,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct RateLimitConfig {
    // TODO: check if instead of String, this could be a Path type.
    pub(crate) api_path_prefix: String,
    pub(crate) method: HttpMethod,
    pub(crate) time_unit: TimeUnit,
    pub(crate) window: u32,
    pub(crate) max_requests: u32,
}

fn get_path_method(
    rate_limit_descriptors: Vec<RateLimitDescriptor>,
) -> (Option<String>, Option<String>) {
    let mut path: Option<String> = None;
    let mut method: Option<String> = None;

    for descriptor in rate_limit_descriptors.into_iter() {
        for Entry { key, value } in descriptor.entries.into_iter() {
            if key == "path" {
                path = Some(value);
            } else if key == "method" {
                method = Some(value)
            }

            if path.is_some() && method.is_some() {
                return (path, method);
            }
        }
    }
    (path, method)
}
#[tonic::async_trait]
impl rate_limit_service_server::RateLimitService for RateLimitServiceImpl {
    async fn should_rate_limit(
        &self,
        request: tonic::Request<RateLimitRequest>,
    ) -> std::result::Result<Response<RateLimitResponse>, tonic::Status> {
        // https://www.envoyproxy.io/docs/envoy/latest/api-v3/service/ratelimit/v3/rls.proto.html#service-ratelimit-v3-ratelimitresponse
        println!("here");
        let path = "dfsdf".to_string();
        let method = HttpMethod::Put;
        let request = request.into_inner();

        let (Some(path), Some(method)) = get_path_method(request.descriptors) else {
            println!("no header and method ");
            return Ok(Response::new(RateLimitResponse {

                overall_code: Code::Ok as i32,
                statuses: vec![],
                quota: None,
                response_headers_to_add: vec![],
                request_headers_to_add: vec![],
                raw_body: vec![],
                dynamic_metadata: None,
            }))
        };
        let Ok(method) = HttpMethod::try_from(method.as_str()) else {
            println!("unsupported http method: {}", method);
            return Ok(Response::new(RateLimitResponse {

                overall_code: Code::Ok as i32,
                statuses: vec![],
                quota: None,
                response_headers_to_add: vec![],
                request_headers_to_add: vec![],
                raw_body: vec![],
                dynamic_metadata: None,
            }))
        };
        println!(
            "will check the rate limit config, method: {:?}, path: {}",
            method, path
        );
        let ok = rand::random::<bool>();
        let mut connection = self.redis_client.get_async_connection().await.unwrap();
        let result: RedisResult<String> = redis::cmd("set")
            .arg("key-3")
            .arg(b"key-4")
            .query_async(&mut connection)
            .await;

        println!("resutlt: {:?}", result);
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
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let ratelimit_config = RateLimitConfig {
        api_path_prefix: "sdfsdf".to_string(),
        method: HttpMethod::Get,
        window: 1,
        time_unit: TimeUnit::M,
        max_requests: 10,
    };
    let cloned = ratelimit_config.clone();
    let rate_limit_configs = vec![ratelimit_config];

    Server::builder()
        .add_service(RateLimitServiceServer::new(RateLimitServiceImpl {
            redis_client: client,
            rate_limit_configs,
        }))
        .serve("0.0.0.0:9000".parse().unwrap())
        .await?;
    Ok(())
}
