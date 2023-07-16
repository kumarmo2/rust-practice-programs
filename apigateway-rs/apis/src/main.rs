mod api_log_layer;
use api_log_layer::ApiLogLayer;
use axum::debug_handler;
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::Json;
use axum::routing::{get, Router};
use prometheus_client::registry::Registry;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;

#[derive(Clone)]
struct AppState {
    prom_registry: Arc<RwLock<Registry>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prom_registry = Arc::new(RwLock::new(Registry::default()));
    let app_state = AppState { prom_registry };
    let router = Router::new();
    let router = router.route("/", get(home_handler));
    let router = router
        .route("/*path", get(default_handler))
        .layer(ServiceBuilder::default().layer(ApiLogLayer::new(app_state.clone())))
        .with_state(app_state.clone());

    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(router.into_make_service())
        .await?;
    Ok(())
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
