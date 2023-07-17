mod api_log_layer;
use api_log_layer::{ApiLogLayer, ApiLogServiceError};
use axum::debug_handler;
use axum::error_handling::HandleErrorLayer;
use axum::extract::{Path, State};
use axum::http::response::Parts;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Json, Response};
use axum::routing::{get, Router};
use prometheus_client::encoding::text::encode;
use prometheus_client::metrics::family::Family;
use prometheus_client::registry::Registry;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;

#[derive(Clone)]
struct AppState {
    prom_registry: Arc<RwLock<Registry>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut prom_registry = Registry::default();

    let counter = Family::default();

    prom_registry.register(
        "http_request_total",
        "Counts number of http requests",
        counter.clone(),
    );
    let app_state = AppState {
        prom_registry: Arc::new(RwLock::new(prom_registry)),
    };
    let router = Router::new();
    let router = router.route("/", get(home_handler));
    let router = router
        .route("/*path", get(default_handler))
        .route("/metrics", get(handle_metrics))
        .layer(
            ServiceBuilder::default()
                .layer(HandleErrorLayer::new(handle_api_log_layer_error))
                .layer(ApiLogLayer::new(counter)),
        )
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(router.into_make_service())
        .await?;
    Ok(())
}

async fn handle_metrics(State(state): State<AppState>) -> (StatusCode, HeaderMap, String) {
    let mut body = String::new();

    let _ = encode(&mut body, state.prom_registry.read().await.deref());
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/openmetrics-text; version=1.0.0; charset=utf-8"
            .parse()
            .unwrap(),
    );
    (StatusCode::OK, headers, body)
}

async fn handle_api_log_layer_error<E>(err: ApiLogServiceError<E>) -> E {
    err.inner
}

#[derive(Serialize, Deserialize)]
struct HomeResponse {
    message: String,
}

#[debug_handler]
async fn default_handler(Path(path): Path<String>, headers: HeaderMap) -> Json<HomeResponse> {
    println!("got request in default handler, path: {}", path);
    println!(">>>>> header starts <<<<<<<");
    for (name, value) in headers.iter() {
        println!("{}: {}", name.as_str(), value.to_str().unwrap());
    }
    println!(">>>>> header ends <<<<<<<");
    Json(HomeResponse {
        message: "hi from default".to_string(),
    })
}

#[debug_handler]
async fn home_handler() -> Json<HomeResponse> {
    println!("got request in home");
    Json(HomeResponse {
        message: "hi from home".to_string(),
    })
}
