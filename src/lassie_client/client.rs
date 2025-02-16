use anyhow::Error;
use iroh_car::CarReader;
use lassie::{Daemon, DaemonConfig};
use std::io::Cursor;
use std::io::Read;
use std::sync::Arc;
use tokio;

pub struct LassieClient {
    daemon: Arc<Daemon>,
    port: u16,
}

impl LassieClient {
    pub fn new() -> Self {
        let daemon = Arc::new(Daemon::start(DaemonConfig::default()).expect("cannot start Lassie"));
        let port = daemon.port();

        Self { daemon, port }
    }

    pub async fn fetch_car(&self, cid: &str) -> Result<Vec<u8>, Error> {
        let port = self.port;
        let url = format!("http://127.0.0.1:{port}/ipfs/{cid}");

        let content = tokio::task::spawn_blocking(move || {
            let response = ureq::get(&url)
                .set("Accept", "application/vnd.ipld.car")
                .call()?;

            let mut content = Vec::new();
            response
                .into_reader()
                .read_to_end(&mut content)
                .expect("cannot read response body");
            anyhow::Ok(content)
        })
        .await??;

        let decoded_content = decode_ipld_to_bytes(content).await?;
        Ok(decoded_content)
    }
}

async fn decode_ipld_to_bytes(data: Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut cursor = Cursor::new(&data);
    let mut car_reader = CarReader::new(&mut cursor).await.unwrap();

    // gets the first block's data without any assumptions about format
    if let Some(block) = car_reader.next_block().await.unwrap() {
        Ok(block.1)
    } else {
        Ok(data)
    }
}
