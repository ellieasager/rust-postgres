use actix_web::{web, Responder};
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::{Serialize, Serializer};
use sqlx::FromRow;
use uuid::Uuid;

use crate::common::AppState;

#[derive(Debug, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub content: String,
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Message", 2)?;
        s.serialize_field("id", &self.id.to_string())?;
        s.serialize_field("content", &self.content)?;
        s.end()
    }
}

#[derive(Deserialize)]
pub struct CreateRequest {
    pub content: String,
}

#[derive(Serialize)]
pub struct ListResponse {
    pub messages: Vec<Message>,
}

pub async fn create_message(
    data: web::Data<AppState>,
    req: web::Json<CreateRequest>,
) -> impl Responder {
    println!("creating a message");

    let id = sqlx::types::Uuid::from_u128(uuid::Uuid::new_v4().as_u128());
    let row: (sqlx::types::Uuid,) =
        sqlx::query_as("insert into messages (id, content) values ($1, $2) returning id")
            .bind(id.to_owned())
            .bind(req.content.to_owned())
            .fetch_one(&data.db_pool)
            .await
            .expect("postgres insertion error");

    println!("row INSERTED: {:?}", row);
    let message = Message {
        id,
        content: req.content.to_owned(),
    };
    println!("created a message");

    web::Json(message)
}

pub async fn list_messages(data: web::Data<AppState>) -> impl Responder {
    println!("listing messages:");

    let select_query = sqlx::query_as::<_, Message>("SELECT id, content FROM messages");
    let messages: Vec<Message> = select_query
        .fetch_all(&data.db_pool)
        .await
        .expect("postgres selection error");
    println!(
        "\n\n=== select messages with query.map...: \n{:?}",
        messages
    );

    web::Json(ListResponse { messages })
}
