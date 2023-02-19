use std::collections::HashMap;
use std::sync::Arc;
// use std::sync::Mutex;

use axum::extract::rejection::JsonRejection;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{
    extract::{Json, State},
    http::Method,
    routing::{any, get, post},
    Router,
};
use serde_json::{json, Value};
use sqlx::pool::Pool;
use sqlx::postgres::Postgres;
use sqlx::Row;
use tokio::sync::Mutex;
mod dto;
use dto::User;

type Db = Arc<Mutex<Pool<Postgres>>>;

#[derive(Clone)]
struct AppState {
    pub(crate) db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // TODO: instead of panicing, return some Result Type from the main.

    let db_pool: Pool<Postgres> =
        match Pool::connect("postgres://postgres:admin@localhost:5432/axum_practice").await {
            Ok(pool) => pool,
            Err(err) => {
                println!("err: {}", err);
                panic!()
            }
        };
    let state = AppState { db: db_pool };
    let app_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(state));

    let app = Router::new()
        .route("/", any(get_method_handler))
        .route("/user", post(create_user))
        .with_state(Arc::clone(&app_state))
        .route("/user/:user_id", get(get_user_by_id))
        .with_state(Arc::clone(&app_state));

    axum::Server::bind(&"127.0.0.1:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[axum_macros::debug_handler]
async fn get_user_by_id(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(user_id): Path<i32>,
) -> (StatusCode, Json<Value>) {
    if user_id < 1 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "err": "Invalid user_id"})),
        );
    }

    let lock = state.lock().await;
    // let mut db = lock.db;

    let optional_user =
        match sqlx::query_as::<_, dto::User>("select * from users.users where id = $1")
            .bind(user_id)
            .fetch_optional(&lock.db)
            .await
        {
            Ok(user) => user,
            Err(err) => {
                println!("some error occurred: {:?}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
            }
        };

    if let Some(user) = optional_user {
        return (StatusCode::OK, Json(json!({ "result": user })));
    }
    (StatusCode::NOT_FOUND, Json(json!({})))
}

#[axum_macros::debug_handler]
async fn create_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(create_user_request): Json<dto::CreateUserRequest>,
) -> (StatusCode, Json<Value>) {
    println!("{:?}", create_user_request);
    let mut lock = state.lock().await;

    let option_user = match sqlx::query_as!(
        User,
        "select id, name from users.users where name = $1",
        &create_user_request.name
    )
    .fetch_optional(&lock.db)
    .await
    {
        Ok(user) => user,
        Err(err) => {
            println!("err: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
        }
    };

    if let Some(_) = option_user {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "err" : "user with same name already exists"})),
        );
    }

    let new_id: i32 = match sqlx::query("insert into users.users(name) values ($1) returning id")
        .bind(create_user_request.name)
        .fetch_one(&lock.db)
        .await
    {
        Ok(row) => row.get("id"),
        Err(err) => {
            println!("err: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
        }
    };
    return (StatusCode::OK, Json(json!({"err": null, "result": new_id})));
}

async fn get_method_handler(method: Method) -> String {
    return format!("http method is: {}", method.as_str());
}
