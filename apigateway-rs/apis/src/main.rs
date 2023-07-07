use axum::debug_handler;
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::Json;
use axum::routing::{get, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut router = Router::new();
    router = router.route("/", get(home_handler));
    router = router.route("/*path", get(default_handler));
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
