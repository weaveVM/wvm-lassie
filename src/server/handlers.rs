use crate::lassie_client::client::LassieClient;
use crate::server::response::{ImportCid, Status};
use crate::utils::wvm::send_envelope;
use axum::{extract::Path, response::Json};
use serde_json::Value;

pub async fn handle_status() -> Json<Value> {
    let res = Status::from("running");
    Json(res)
}

pub async fn handle_import_cid(Path(cid): Path<String>) -> Json<Value> {
    let daemon = LassieClient::new();
    let data = daemon.fetch_car(&cid).await.unwrap_or_default();
    let envelope_tx = send_envelope(data.clone()).await.unwrap_or_default();
    let res = ImportCid::from(&envelope_tx, data);
    Json(res)
}
