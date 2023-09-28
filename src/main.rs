use actix_web::{get, web, App, HttpServer, Responder};
use std::env;
use tokio_postgres::{NoTls, Error};
use tokio::runtime;
mod routes;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_url = "postgresql://postgres:5657@localhost/api_gateway";

    let (client, connection) = tokio_postgres::connect(db_url, NoTls).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error connecting to the database: {}", e);
        }
    });

    let query = "SELECT id, name FROM usuario";
    for row in client.query(query, &[]).await? {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        println!("ID: {}, Name: {}", id, name);
    }

    Ok(())

    let HOST = "127.0.0.1";
    let PORT = "8080";

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