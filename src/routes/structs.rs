use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct create_transfer {
    pub doc_cliente_origem: i32,
    pub doc_cliente_cest: i32,
    pub nome_cliente_origem: String,
    pub nome_cliente_des: String,
    pub banco_ori: String,
    pub banco_dest: String,
    pub ValorTransf: i32,
    pub DataHora: String,
}

#[derive(Deserialize, Serialize)]
pub struct update_transfer {
    pub id: i32,
    pub banco_destino: String,
    pub Valor_transf: i32,
    pub DataHora: String,
}

#[derive(Deserialize, Serialize)]
pub struct delete_transfer {
    pub id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct create_transfer_pix {
    pub chave: String,
    pub tipo: String,
    pub transfer_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct update_transfer_pix {
    pub id: i32,
    pub chave: String,
}