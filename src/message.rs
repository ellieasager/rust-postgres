use actix_web::{web, Responder, ResponseError};
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::{Serialize, Serializer};
use sqlx::FromRow;
use uuid::Uuid;

use crate::common::AppState;

// `FromRow` is needed so that we can use method `fetch_all` to fetch messages from db
#[derive(Debug, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub content: String,
}

// This is needed to satisfy the `Responder` trait
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

#[derive(Debug)]
pub struct MyError(String);

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl ResponseError for MyError {}

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
    let response: Result<(sqlx::types::Uuid,), sqlx::Error> =
    sqlx::query_as("insert into messages (id, content) values ($1, $2) returning id")
        .bind(id.to_owned())
        .bind(req.content.to_owned())
        .fetch_one(&data.db_pool)
        .await;

    match response {
        Ok(row) => {
            println!("row INSERTED: {:?}", row);
            let message = Message {
                id,
                content: req.content.to_owned(),
            };
            Ok(web::Json(message))
        }, 
        Err(sqlx_error) => {
            println!(
                "\n\n=== error creating a message \n{:?}",
                sqlx_error.to_string());
            Err(MyError(sqlx_error.to_string()))
        },
    }
}

pub async fn list_messages(data: web::Data<AppState>) -> impl Responder {
    println!("listing messages:");

    let select_query = sqlx::query_as::<_, Message>("SELECT id, content FROM messages");
    let response = select_query.fetch_all(&data.db_pool).await;

    match response {
        Ok(messages) => {
            println!(
                "\n\n=== select messages: \n{:?}",
                messages);
            Ok(web::Json(ListResponse { messages }))
        },
        Err(sqlx_error) => {
            println!(
                "\n\n=== error selecting messages: \n{:?}",
                sqlx_error.to_string());
            Err(MyError(sqlx_error.to_string()))
        },
    }
}
