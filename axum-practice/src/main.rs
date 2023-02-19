use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use axum::http::StatusCode;
use axum::{
    extract::{Json, State},
    http::Method,
    routing::{any, get, post},
    Router,
};
use serde_json::{json, Value};
mod dto;

type UserDb = Arc<Mutex<HashMap<i32, dto::User>>>;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let user_db: UserDb = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", any(get_method_handler))
        .route("/user", post(create_user))
        .with_state(user_db);

    axum::Server::bind(&"127.0.0.1:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// async fn create_user(Json(user): Json<dto::CreateUserRequest>, State(user_db): State<UserDb>) {
#[axum_macros::debug_handler]
async fn create_user(
    State(user_db): State<UserDb>,
    Json(create_user_request): Json<dto::CreateUserRequest>,
) -> Json<Value> {
    println!("{:?}", create_user_request);
    let mut lock = user_db.lock().unwrap();

    let user_with_name_already_exists = lock.values().any(|user| {
        user.name
            .eq_ignore_ascii_case(&create_user_request.name.trim())
    });

    if let true = user_with_name_already_exists {
        return Json(json!({ "err" : "user with same name already exists"}));
        // return (
        // StatusCode::OK,
        // Some(Json(json!({ "err" : "user with same name already exists"}))),
        // );
    }

    let new_id = match lock.values().map(|u| u.id).max() {
        Some(id) => id + 1,
        None => 1,
    };

    let user = dto::User {
        id: new_id,
        name: create_user_request.name,
    };

    lock.insert(new_id, user);
    return Json(json!({"err": null, "result": new_id}));
}

async fn get_method_handler(method: Method) -> String {
    return format!("http method is: {}", method.as_str());
}
