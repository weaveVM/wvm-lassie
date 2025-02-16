use lassie::Daemon;
use lassie::DaemonConfig;
use ureq;
use std::io::Read;


pub fn main() {
  let daemon = Daemon::start(DaemonConfig::default()).expect("cannot start Lassie");
  let port = daemon.port();

  let url = format!("http://127.0.0.1:{port}/ipfs/bafybeib36krhffuh3cupjml4re2wfxldredkir5wti3dttulyemre7xkni");
  println!("{:?}", url);
  let response = ureq::get(&url)
      .set("Accept", "application/vnd.ipld.car")
      .call()
      .expect("error");
  
      let mut content = Vec::new();
      response
          .into_reader()
          .read_to_end(&mut content)
          .expect("cannot read response body");
  
      println!("{:?}", content);
    }