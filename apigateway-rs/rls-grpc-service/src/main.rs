#![allow(unused_variables, unused_imports)]
pub(crate) mod envoy;

pub(crate) mod validate;

pub(crate) mod xds;

pub(crate) mod udpa;

#[cfg(test)]
mod test;
pub(crate) mod throttler;

use consulrs::api::service::requests::{RegisterServiceRequest, RegisterServiceRequestBuilder};
use consulrs::client::{ConsulClient, ConsulClientSettings, ConsulClientSettingsBuilder};
use consulrs::service::{deregister, register};
use ctrlc;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::mpsc::channel;

use envoy::{
    extensions::common::ratelimit::v3::{rate_limit_descriptor::Entry, RateLimitDescriptor},
    service::ratelimit::v3::{
        rate_limit_service_server::{self, RateLimitServiceServer},
        RateLimitRequest, RateLimitResponse,
    },
};

use serde::{Deserialize, Serialize};
use throttler::Throttler;
use tonic::{transport::Server, Response};

use crate::envoy::{
    config::core::v3::HeaderValue, service::ratelimit::v3::rate_limit_response::Code,
};

struct RateLimitServiceImpl {
    // TODO: use some sort of connection pooling.
    redis_client: redis::Client,
    // rate_limit_configs: Vec<RateLimitConfig>,
    throttler: Throttler,
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

impl HttpMethod {
    pub(crate) fn to_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "get",
            HttpMethod::Post => "post",
            HttpMethod::Put => "put",
            HttpMethod::Delete => "delete",
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

impl RateLimitConfig {
    pub(crate) fn get_window_in_seconds(&self) -> u32 {
        self.window * self.time_unit as u32
    }

    pub(crate) fn get_config_key_for_client(&self, client_id: &str) -> String {
        format!(
            "client={}|path={}|method={}|window={}",
            client_id,
            self.api_path_prefix,
            self.method.to_str(),
            self.get_window_in_seconds()
        )
    }
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
        // println!(" method: {:?}, path: {}", method, path);
        let should_throttle = match self
            .throttler
            .should_throttle(path.as_str(), method, "cvbcvb", &self.redis_client)
            .await
        {
            Ok(result) => result,
            Err(err) => {
                println!("error from should_throttle: {}", err);
                false
            }
        };
        println!("should_throttle: {}", should_throttle);
        match should_throttle {
            true => Ok(Response::new(RateLimitResponse {
                overall_code: Code::OverLimit as i32,
                statuses: vec![],
                quota: None,
                response_headers_to_add: vec![HeaderValue {
                    key: "sdf".to_string(),
                    value: "cvxcv".to_string(),
                }],
                request_headers_to_add: vec![HeaderValue {
                    key: "k1".to_string(),
                    value: "v1".to_string(),
                }],
                raw_body: vec![],
                dynamic_metadata: None,
            })),
            false => Ok(Response::new(RateLimitResponse {
                overall_code: Code::Ok as i32,
                statuses: vec![],
                response_headers_to_add: vec![HeaderValue {
                    key: "poiuy".to_string(),
                    value: "cvxcv".to_string(),
                }],
                request_headers_to_add: vec![HeaderValue {
                    key: "k2".to_string(),
                    value: "v2".to_string(),
                }],
                raw_body: vec![],
                dynamic_metadata: None,
                quota: None,
            })),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: deregistering.

    // RegisterServiceRequest{ features: todo!(),
    // address: todo!(), check: todo!(), checks: todo!(), connect: todo!(), enable_tag_override: todo!(), id: todo!(), kind: todo!(), meta: todo!(), name: todo!(), ns: todo!(), port: todo!(), proxy: todo!(), tagged_addresses: todo!(), tags: todo!(), weights: todo!() }
    // ConsulClientSettings{}
    let consul_client = ConsulClient::new(ConsulClientSettingsBuilder::default().build()?)?;
    register(
        &consul_client,
        "rls-service",
        Some(
            &mut (RegisterServiceRequestBuilder::default()
                .id("rls-service-1")
                .port(9000 as u64))
            .tags(vec!["tag2".to_string(), "tag1".to_string()]),
        ),
    )
    .await?;

    let (tx, rx) = channel();

    ctrlc::set_handler(move || tx.send(()).unwrap())?;

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let ratelimit_config = RateLimitConfig {
        api_path_prefix: "/api/".to_string(),
        method: HttpMethod::Get,
        window: 10,
        time_unit: TimeUnit::S,
        max_requests: 3,
    };
    let mut rate_limit_configs = vec![ratelimit_config];
    let ratelimit_config = RateLimitConfig {
        api_path_prefix: "/".to_string(),
        method: HttpMethod::Get,
        window: 2,
        time_unit: TimeUnit::M,
        max_requests: 7,
    };
    rate_limit_configs.push(ratelimit_config);

    tokio::spawn(async {
        Server::builder()
            .add_service(RateLimitServiceServer::new(RateLimitServiceImpl {
                redis_client: client,
                throttler: Throttler::new(rate_limit_configs),
            }))
            .serve("0.0.0.0:9000".parse().unwrap())
            .await
            .unwrap()
    });
    rx.recv().unwrap();
    println!("ctrl-c received");
    // service::deregister(&client, id, opts)
    deregister(&consul_client, "rls-service-1", None).await?;
    Ok(())
}
