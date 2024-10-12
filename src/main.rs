use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod common;
mod message;

use common::AppState;
use message::{create_message, list_messages};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server is running");

    dotenv().ok();

    // let db_url = env::var("DATABASE_URL").expect("Database url not set in .env file");
    let db_url = db_url();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await
        .expect("postgres connection error");

    let data = web::Data::new(AppState { db_pool: pool });
    println!("Connection to the database established!");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(hello) // sanity check
            .service(create_message)
            .service(list_messages)
    })
    .bind(address())?
    .run()
    .await
}

fn db_url() -> String {
    let db_host = env::var("PG_HOST").unwrap_or_else(|_| "localhost".into());
    let db_user = env::var("PG_USER").unwrap_or_else(|_| "postgres".into());
    let db_password = env::var("PG_PASSWORD").unwrap_or_else(|_| "postgres".into());
    let db_name = env::var("PG_DBNAME").unwrap_or_else(|_| "messages".into());
    format!("postgres://{db_user}:{db_password}@{db_host}:5432/{db_name}")
}

fn address() -> String {
    env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".into())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
