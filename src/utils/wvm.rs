use crate::utils::get_env_var::get_env_var;
use anyhow::Error;
use bundler::utils::core::bundle::Bundle;
use bundler::utils::core::envelope::Envelope;
use bundler::utils::core::tags::Tag;

pub async fn send_envelope(envelope_data: Vec<u8>) -> Result<String, Error> {
    let private_key = get_env_var("WVM_PK")?;

    let mut envelopes: Vec<Envelope> = vec![];

    let tags = vec![
        Tag::new(
            "Content-Type".to_string(),
            "application/octet-stream".to_string(),
        ),
        Tag::new("protocol".to_string(), "filecoin-importer".to_string()),
        Tag::new("client".to_string(), "wvm-lassie".to_string()),
    ];

    let envelope = Envelope::new()
        .data(Some(envelope_data))
        .target(None)
        .tags(Some(tags.clone()))
        .build()
        .unwrap();
    envelopes.push(envelope);

    let bundle_tx = Bundle::new()
        .private_key(private_key)
        .envelopes(envelopes)
        .build()
        .expect("REASON")
        .propagate()
        .await
        .unwrap();

    Ok(bundle_tx)
}
