pub mod lassie_client;
pub mod server;
pub mod utils;
use crate::server::handlers::{handle_import_cid, handle_status};
use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

// let cid = "bafybeib36krhffuh3cupjml4re2wfxldredkir5wti3dttulyemre7xkni";

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // load secrets from Secrets.toml into env var;
    secrets.into_iter().for_each(|(key, val)| {
        println!("{:?} {:?}", key, val);
        std::env::set_var(key, val);
    });
    let router = Router::new()
        .route("/", get(handle_status))
        .route("/import/:cid", get(handle_import_cid));
    Ok(router.into())
}
