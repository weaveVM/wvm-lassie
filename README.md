<p align="center">
  <a href="https://wvm.dev">
    <img src="https://raw.githubusercontent.com/weaveVM/.github/main/profile/bg.png">
  </a>
</p>

## About
`wvm-lassie` is an IPFS/Filecoin data importer to WeaveVM -- make your Filecoin data live forever.

This tool provides a simple ETL to retrieve content from the Filecoin network using CIDs. It spin up a [Lassie daemon](https://github.com/CheckerNetwork/rusty-lassie) for each retrieval request, downloads the content of the CAR file, and then retrieve the IPLD blocks. Then, data is loaded to WeaveVM tagged with `application/octet-stream` MIME type.

### Key concepts

- **CID**: Content Identifiers are unique identifiers for content in the Filecoin/IPFS network.
- **CAR**: Content Addressable aRchives are a format for storing IPLD data.
- **IPLD**: InterPlanetary Linked Data is the data model used by IPFS and Filecoin.

## REST API

### Import cid

- API endpoint: (159.65.81.229)[http://159.65.81.229/]

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