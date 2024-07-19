use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct Text {
	id: i64,
	text: String,
}

#[derive(Deserialize)]
struct CreateRequest {
    text: String,
}

async fn create(req: web::Json<CreateRequest>) -> impl Responder {
    println!("create");
    let text = Text {
        id: 1,
        text: req.text.to_owned(),
    };
    web::Json(text)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server is running");

    HttpServer::new(move || {
        App::new()
            .route("create", web::post().to(create))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
