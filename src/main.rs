pub mod lassie_client;
pub mod server;
pub mod utils;

use crate::server::handlers::{handle_import_cid, handle_status};
use anyhow::Result;
use axum::{routing::get, Router};
use bundler::utils::env_var::get_env_key;
use std::net::SocketAddr;
use tokio::net::TcpListener;

// let cid = "bafybeib36krhffuh3cupjml4re2wfxldredkir5wti3dttulyemre7xkni";
#[tokio::main]
async fn main() -> Result<()> {
    let port = get_env_key("PORT".to_string()).unwrap_or("3000".to_string()).parse::<u16>()?;
    let app = Router::new()
        .route("/", get(handle_status))
        .route("/import/:cid", get(handle_import_cid));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;
    println!("Server starting on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}