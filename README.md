<p align="center">
  <a href="https://wvm.dev">
    <img src="https://raw.githubusercontent.com/weaveVM/.github/main/profile/bg.png">
  </a>
</p>

## About
`wvm-lassie` is an IPFS/Filecoin data importer to WeaveVM -- make your Filecoin data live forever.

## REST API



### Import cid

- API endpoint: `TBD`

```bash
GET /import/:cid
```

#### Response

```rust
pub struct ImportCid {
    pub wvm_bundle_txid: String,
    pub data: Vec<u8>,
}
```

## License 
This project is license under the [MIT License](./LICENSE)