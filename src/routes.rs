use actix_web::{post, get, web, App, HttpServer, HttpResponse, Responder, Result};
use std::{collections::HashMap, path};
use serde_json::json;
mod structs;

// Login Route
#[post("/login")]
pub async fn login(req_body: String) -> impl Responder {
    let data: structs::UserData = serde_json::from_str(&req_body).unwrap();
    let username: String = data.username;
    let password: String = data.password;
    
    let mut vec:Vec<structs::ResponseMessage> = Vec::new();
    
    let client = reqwest::Client::new();
    match client.get("http://localhost:3000/users/651e05700934394373efe58d")
    .send()
    .await
    .expect("Failed to fetch")
        .json::<structs::UserData>()
        .await {
            Ok(res) => {
                let mut found: bool = false;
                let mut correct_password: bool = false;
                
                if res.username == username {
                    found = true;
                    if res.password == password {
                        correct_password = true;
                    }
                }

                if found == true {
                    if correct_password == true {
                        vec.push(
                            structs::ResponseMessage {
                                message: "Alright!".to_string(), 
                                code: 200, 
                                logged: "true".to_string()
                        });
                    } else {
                        vec.push(
                            structs::ResponseMessage {
                                message: "Password incorrect!".to_string(), 
                                code: 200, 
                                logged: "false".to_string()
                        });
                    }
                } else {
                    vec.push(
                        structs::ResponseMessage {
                            message: "Username not found!".to_string(),
                            code: 200, 
                            logged: "false".to_string()
                    });
                }
            },
            Err(erro) => {
                vec.push(
                    structs::ResponseMessage {
                        message: erro.to_string(),
                        code: 500, 
                        logged: "false".to_string()
                    }
                )  
            }   
        }
        
        return web::Json(vec);
}

//Transfer Route
pub async fn makeTED(info: web::Json<structs::make_TED>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("id", info.id.to_string());
    map.insert("conta", info.conta.to_string());

    let res = reqwest::Client::new()
    .post("https://localhost:3000/makeTED/651e05700934394373efe58d")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test {
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

pub async fn newTED(info: web::Json<structs::new_TED>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("agencia", info.agencia.to_string());
    map.insert("conta", info.conta.to_string());
    map.insert("transfer_id", info.transfer_id.to_string());

    let res = reqwest::Client::new()
    .post("https://localhost:3000/new_TED/651e05700934394373efe58d")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test {
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

pub async fn makePIX(info: web::Json<structs::make_PIX>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("id", info.id.to_string());
    map.insert("chave", info.chave.to_string());

    let res = reqwest::Client::new()
    .post("https://localhost:3000/doTransfer/651e05700934394373efe58d")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test {
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

pub async fn newPIX(info: web::Json<structs::new_PIX>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("transfer_id", info.transfer_id.to_string());
    map.insert("chave", info.chave.to_string());
    map.insert("tipo", info.tipo.to_string());
    map.insert("value", info.value.to_string());

    let res = reqwest::Client::new()
    .post("https://localhost:3000/doTransfer/651e05700934394373efe58d")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test{
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

pub async fn undoTransfer(info: web::Json<structs::update_transfer>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("id", info.id.to_string());

    let res = reqwest::Client::new()
    .post("https://localhost:3000/doTransfer/651e05700934394373efe58d")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test {
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}


pub async fn doTransfer(info: web::Json<structs::update_transfer>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("id", info.id.to_string());
    map.insert("bancoDestino", info.bancoDestino.to_string());
    map.insert("ValorTransf", info.ValorTransf.to_string());
    map.insert("DataHora", info.DataHora.to_string());

    let res = reqwest::Client::new()
    .post("https://localhost:3000/doTransfer/651e05700934394373efe58d")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test{
        status: "OK".to_string(),
    };
    
    println!("RECEBA! MELHOR DO MUNDO!");

    Ok(web::Json(obj))
}