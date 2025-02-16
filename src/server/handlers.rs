use crate::lassie_client::client::LassieClient;
use crate::utils::wvm::send_envelope;
use axum::{extract::Path, response::Json};
use serde_json::{json, Value};

pub async fn handle_status() -> Json<Value> {
    Json(json!({"status": "running"}))
}

pub async fn handle_import_cid(Path(cid): Path<String>) -> Json<Value> {
    let daemon = LassieClient::new();
    let data = daemon.fetch_car(&cid).await.unwrap_or_default();
    let envelope_tx = send_envelope(data.clone()).await.unwrap_or_default();
    Json(json!({"wvm_txid": envelope_tx, "data": data}))
}
