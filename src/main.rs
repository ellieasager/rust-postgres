use actix_web::{
    body::BoxBody, http::header::ContentType, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use sqlx::Postgres;
use std::env;

#[derive(Debug, Serialize, FromRow)]
struct Text {
    id: i64,
    content: String,
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
    texts: Vec<Text>,
}

impl Responder for ListResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

async fn create(data: web::Data<AppState>, req: web::Json<CreateRequest>) -> impl Responder {
    println!("creating a text");

    let row: (i64,) = sqlx::query_as("insert into texts (content) values ($1) returning id")
        .bind(req.content.to_owned())
        .fetch_one(&data.db_pool)
        .await
        .expect("postgres insertion error");

    println!("row INSERTED: {:?}", row);
    let text = Text {
        id: row.0,
        content: req.content.to_owned(),
    };
    println!("created a text");

    web::Json(text)
}

async fn list(data: web::Data<AppState>) -> impl Responder {
    println!("listing texts:");

    let select_query = sqlx::query_as::<_, Text>("SELECT id, content FROM texts");
    let texts: Vec<Text> = select_query
        .fetch_all(&data.db_pool)
        .await
        .expect("postgres fancy selection error");
    println!("\n=== select texts with query.map...: \n{:?}", texts);

    web::Json(ListResponse { texts })
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
    CREATE TABLE IF NOT EXISTS texts (
      id bigserial,
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
