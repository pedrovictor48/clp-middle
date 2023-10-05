use serde::{Deserialize, Serialize};

// Login Route

#[derive(Serialize, Deserialize, Debug)]
pub struct Account{
    pub login: String,
    pub password: String,
    pub cpf: String,
    pub name: String, 
    pub balance: String,
    pub id: i64,
}
#[derive(Debug)]
pub enum Response {
    Results(Vec<AccountDetails>),
    Error(Vec<reqwest::Error>),
}

#[derive(Serialize)]
pub struct Response{
    pub code: i32,
    pub message: String,
    pub logged: String,
}

// Transfer Route

#[derive(Deserialize, Serialize)]
pub struct doTransfer {
    pub origin_account: String,
    pub target_account_or_pix_key: i32,
    pub value: f32,
    pub transference_type: i32,
    pub selected_date: String,
    pub hour: String,
}


#[derive(Deserialize, Serialize)]
pub struct new_PIX {
    pub chave: String,
    pub tipo: String,
    pub transfer_id: i32,
    pub value: f64,
}

#[derive(Deserialize, Serialize)]
pub struct make_PIX{
    pub id: i32,
    pub chave: String,
}

#[derive(Deserialize, Serialize)]
pub struct new_TED{
    pub agencia: String,
    pub conta: String,
    pub transfer_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct make_TED {
    pub id: i32,
    pub conta: String,
}

#[derive(Serialize)]
pub struct Test{
    pub status: String,
} 
