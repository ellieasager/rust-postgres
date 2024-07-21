use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use std::env;

mod common;
mod message;
mod simple_message;

use common::AppState;
use message::{create_message, list_messages};
use simple_message::{create_simple_message, list_simple_messages};

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

    init_messages_table(&pool).await;
    init_simple_messages_table(&pool).await;

    let data = web::Data::new(AppState { db_pool: pool });
    println!("Connection to the database established!");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route(
                "simple_messages/create",
                web::post().to(create_simple_message),
            )
            .route("simple_messages/list", web::get().to(list_simple_messages))
            .route("messages/create", web::post().to(create_message))
            .route("messages/list", web::get().to(list_messages))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn init_simple_messages_table(pool: &Pool<Postgres>) -> () {
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS simple_messages (
      id bigserial,
      content text
    );"#,
    )
    .execute(pool)
    .await
    .expect("postgres simple_messages_table creation error");
}

async fn init_messages_table(pool: &Pool<Postgres>) -> () {
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS messages (
      id uuid,
      content text
    );"#,
    )
    .execute(pool)
    .await
    .expect("postgres messages_table creation error");
}
