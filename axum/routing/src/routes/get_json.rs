use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "I'm in Data".to_owned(),
        count: 1123,
        username: "clemado1".to_owned(),
    };

    Json(data)
}