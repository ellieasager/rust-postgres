use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct SimpleMessage {
    pub id: i64,
    pub content: String,
}

#[derive(Serialize)]
pub struct ListSimpleResponse {
    pub messages: Vec<SimpleMessage>,
}
