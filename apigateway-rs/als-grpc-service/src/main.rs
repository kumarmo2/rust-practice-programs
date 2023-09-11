#![allow(unused_variables, dead_code)]
mod envoy;
mod log_service;
mod xds;

use std::{ops::Deref, sync::Arc};

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::get,
    Router,
};
use prometheus_client::{encoding::text::encode, metrics::family::Family, registry::Registry};

use crate::{
    envoy::service::accesslog::v3::access_log_service_server::AccessLogServiceServer,
    log_service::CustomAccessLogService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut prom_registry = Registry::default();

    let counter = Family::default();

    prom_registry.register(
        "hcm_access_logs_als",
        "HCM access logs via ALS(grps)",
        counter.clone(),
    );

    let prom_registry = Arc::new(prom_registry);

    let grpc_server = tokio::spawn(async move {
        // TODO: remove unwrap.
        let grpc_server_task = tonic::transport::Server::builder()
            .add_service(AccessLogServiceServer::new(CustomAccessLogService {
                http_request_count_metrics: counter.clone(),
            }))
            .serve("0.0.0.0:9001".parse().unwrap());

        grpc_server_task.await.unwrap();
    });

    let http_server = tokio::spawn(async move {
        let router = Router::new()
            .route("/metrics", get(handle_metrics))
            .with_state(prom_registry);

        let http_server_task =
            axum::Server::bind(&"0.0.0.0:9002".parse().unwrap()).serve(router.into_make_service());
        http_server_task.await.unwrap();
    });

    let x = tokio::join!(grpc_server, http_server);

    Ok(())
}

async fn handle_metrics(
    State(prom_registry): State<Arc<Registry>>,
) -> (StatusCode, HeaderMap, String) {
    let mut body = String::new();

    let _ = encode(&mut body, prom_registry.deref());
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/openmetrics-text; version=1.0.0; charset=utf-8"
            .parse()
            .unwrap(),
    );
    (StatusCode::OK, headers, body)
}
