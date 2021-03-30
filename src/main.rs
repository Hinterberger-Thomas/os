use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use serde::{*};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/hey", web::get().to(current_temperature))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Serialize)]
struct Measurement {
    temperature: f32,
}

async fn current_temperature() -> impl Responder {
    web::Json(Measurement { temperature: 42.3 })
}
