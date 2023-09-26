use actix_web::{post, get, web, App, HttpServer, HttpResponse, Responder, Result};
use std::{collections::HashMap, path};
use serde_json::json;
mod structs;

pub async fn updateTransferTedDoc(info: web::Json<structs::update_transfer_ted_doc>) -> Result<impl Responder> {

    let mut map = HashMap::new();
    map.insert("id", info.id.to_string());
    map.insert("conta", info.conta.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let ok_obj = structs::Test {
        status: "OK".to_string(),
    };
    
    Ok(web::Json(ok_obj))
}

pub async fn createTransferTedDoc(info: web::Json<structs::create_transfer_ted_doc>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("agencia", info.agencia.to_string());
    map.insert("conta", info.conta.to_string());
    map.insert("transfer_id", info.transfer_id.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test {
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

pub async fn updateTransferPix(info: web::Json<structs::update_transfer_pix>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("id", info.id.to_string());
    map.insert("chave", info.chave.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test {
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

pub async fn createTransferPix(info: web::Json<structs::create_transfer_pix>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("transfer_id", info.transfer_id.to_string());
    map.insert("chave", info.chave.to_string());
    map.insert("tipo", info.tipo.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test{
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}