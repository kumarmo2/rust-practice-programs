use axum::debug_handler;
use axum::response::Json;
use axum::routing::{get, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut router = Router::new();
    router = router.route("/", get(default_handler));
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
async fn default_handler() -> Json<HomeResponse> {
    println!("got request in home");
    Json(HomeResponse {
        message: "hi from home".to_string(),
    })
}
