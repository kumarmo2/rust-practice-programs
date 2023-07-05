pub(crate) mod envoy;

pub(crate) mod validate;

pub(crate) mod xds;

pub(crate) mod udpa;

use envoy::service::ratelimit::v3::{
    rate_limit_service_server, RateLimitRequest, RateLimitResponse,
};
use tonic::Code;
struct RateLimitServiceImpl {}

#[tonic::async_trait]
impl rate_limit_service_server::RateLimitService for RateLimitServiceImpl {
    async fn should_rate_limit(
        &self,
        request: tonic::Request<RateLimitRequest>,
    ) -> std::result::Result<tonic::Response<RateLimitResponse>, tonic::Status> {
        Err(tonic::Status::new(Code::Ok, "not passed"))
        // todo!()
    }
}

#[tokio::main]
async fn main() {
    // let x = envoy::service::ratelimit::v3::RateLimitRequest {};
}
