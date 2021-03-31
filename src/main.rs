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
            .route("/sev/{hey}", web::get().to(greeting))
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
    ret_json()
}

fn ret_json()->web::Json<Measurement>{
    let x = 0.0;
    return web::Json(Measurement { temperature: x });
}

async fn greeting(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("hey").unwrap_or("World");
    format!("Hello {}!", &name)
}