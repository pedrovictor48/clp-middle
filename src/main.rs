use actix_web::{get, web, App, HttpServer, Responder};
use std::env;
mod routes;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("BEM VINDO {name}! AO BANCO DO PIX!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let HOST = "127.0.0.1";
    let PORT = "8080";

    println!("VENHA FAZER O PIX AGORA!!!");

    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(routes::login)
            // .route("/createTransfer", web::post().to(routes::createTransfer))
            .route("/doTransfer", web::post().to(routes::doTransfer))
            .route("/undoTransfer", web::post().to(routes::undoTransfer))
            .route("/newPIX", web::post().to(routes::newPIX))
            .route("/makePIX", web::post().to(routes::makePIX))
            .route("/newTED", web::post().to(routes::newTED))
            .route("/makeTED", web::post().to(routes::makeTED))
    })
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}