use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ImportCid {
    pub wvm_bundle_txid: String,
    pub data: Vec<u8>,
}

impl Status {
    pub fn from(status: &str) -> Value {
        let res = Self {
            status: status.to_string(),
        };

        serde_json::to_value(&res).unwrap_or_default()
    }
}

impl ImportCid {
    pub fn from(wvm_bundle_txid: &str, data: Vec<u8>) -> Value {
        let res = Self {
            wvm_bundle_txid: wvm_bundle_txid.to_string(),
            data,
        };

        serde_json::to_value(&res).unwrap_or_default()
    }
}
