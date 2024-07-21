use actix_web::{web, Responder};
use serde::Serialize;
use sqlx::FromRow;

use crate::common::{AppState, CreateRequest};

#[derive(Debug, Serialize, FromRow)]
pub struct SimpleMessage {
    pub id: i64,
    pub content: String,
}

#[derive(Serialize)]
pub struct ListSimpleResponse {
    pub messages: Vec<SimpleMessage>,
}

pub async fn create_simple_message(
    data: web::Data<AppState>,
    req: web::Json<CreateRequest>,
) -> impl Responder {
    println!("creating a simple_message");

    let row: (i64,) =
        sqlx::query_as("insert into simple_messages (content) values ($1) returning id")
            .bind(req.content.to_owned())
            .fetch_one(&data.db_pool)
            .await
            .expect("postgres insertion error");

    println!("row INSERTED: {:?}", row);
    let simple_message = SimpleMessage {
        id: row.0,
        content: req.content.to_owned(),
    };
    println!("created a simple_message");

    web::Json(simple_message)
}

pub async fn list_simple_messages(data: web::Data<AppState>) -> impl Responder {
    println!("listing simple_messages:");

    let select_query =
        sqlx::query_as::<_, SimpleMessage>("SELECT id, content FROM simple_messages");
    let simple_messages: Vec<SimpleMessage> = select_query
        .fetch_all(&data.db_pool)
        .await
        .expect("postgres selection error");
    println!(
        "\n\n=== select simple_messages with query.map...: \n{:?}",
        simple_messages
    );

    web::Json(ListSimpleResponse {
        messages: simple_messages,
    })
}
