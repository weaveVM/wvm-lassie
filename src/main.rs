pub mod lassie_client;
use axum::{routing::get, Json, Router};
use lassie_client::client::LassieClient;
use serde_json::{json, Value};

pub async fn handle_weave_gm() -> Json<Value> {
    let client = LassieClient::new();
    let cid = "bafybeib36krhffuh3cupjml4re2wfxldredkir5wti3dttulyemre7xkni";
    let data = client.fetch_car(cid).await.unwrap();
    Json(json!({"data": data}))
}
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(handle_weave_gm));
    Ok(router.into())
}
