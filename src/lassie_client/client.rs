use anyhow::{Error, Ok};
use lassie::{Daemon, DaemonConfig};
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

        tokio::task::spawn_blocking(move || {
            let response = ureq::get(&url)
                .set("Accept", "application/vnd.ipld.car")
                .call()?;

            let mut content = Vec::new();
            response
                .into_reader()
                .read_to_end(&mut content)
                .expect("cannot read response body");
            Ok(content)
        })
        .await?
        .map_err(Into::into)
    }
}
