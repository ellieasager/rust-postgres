use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use sqlx::Postgres;
use std::env;
use uuid::Uuid;

#[derive(Debug, FromRow)]
struct Message {
    id: Uuid,
    content: String,
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

struct AppState {
    db_pool: Pool<Postgres>,
}

#[derive(Deserialize)]
struct CreateRequest {
    content: String,
}

#[derive(Serialize)]
struct ListResponse {
    messages: Vec<Message>,
}

async fn create(data: web::Data<AppState>, req: web::Json<CreateRequest>) -> impl Responder {
    println!("creating a message");

    let id = sqlx::types::Uuid::from_u128(uuid::Uuid::new_v4().as_u128());
    let row: (sqlx::types::Uuid,) = sqlx::query_as(
        "insert into messages (id, content) values ($1, $2) returning id AS \"id: Uuid\"",
    )
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

async fn list(data: web::Data<AppState>) -> impl Responder {
    println!("listing messages:");

    let select_query = sqlx::query_as::<_, Message>("SELECT id, content FROM messages");
    let messages: Vec<Message> = select_query
        .fetch_all(&data.db_pool)
        .await
        .expect("postgres fancy selection error");
    println!("\n=== select messages with query.map...: \n{:?}", messages);

    web::Json(ListResponse { messages })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server is running");

    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Database url not set in .env file");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await
        .expect("postgres connection error");

    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS messages (
      id uuid,
      content text
    );"#,
    )
    .execute(&pool)
    .await
    .expect("postgres table creation error");

    let data = web::Data::new(AppState { db_pool: pool });
    println!("Connection to the database established!");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("create", web::post().to(create))
            .route("list", web::get().to(list))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
