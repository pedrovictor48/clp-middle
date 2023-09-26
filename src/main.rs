use actix_web::{get, web, App, HttpServer, Responder};
use std::env;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    // let HOST = env::var("HOST").expect("Host not set");
    // let PORT = env::var("PORT").expect("Port not set");
    
    let HOST = "127.0.0.1";
    let PORT = "8080";

    println!("Project running!");

    HttpServer::new(|| {
        App::new()
            .route("/createTransferTedDoc", web::post().to(routes::createTransferTedDoc))
            .route("/updateTransferTedDoc", web::post().to(routes::updateTransferTedDoc))
            .route("/createTransferPix", web::post().to(routes::createTransferPix))
            .route("/updateTransferPix", web::post().to(routes::updateTransferPix))
    })
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}