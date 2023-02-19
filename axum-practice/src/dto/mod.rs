use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub(crate) struct User {
    pub(crate) name: String,
    pub(crate) id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CreateUserRequest {
    pub(crate) name: String,
}
