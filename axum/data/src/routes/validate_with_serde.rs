use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct RequestUser {
    username: Option<String>,
    password: String,
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) {
    dbg!(user);
}