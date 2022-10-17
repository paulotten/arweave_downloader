use anyhow::{bail, Result};
use hyper::{
    body::Buf,
    client::{connect::dns::GaiResolver, HttpConnector},
    Body, Response,
};
use hyper_tls::HttpsConnector;
use serde::Deserialize;

type Client = hyper::Client<HttpsConnector<HttpConnector<GaiResolver>>, Body>;

const BASE_URI: &str = "https://arweave.net";
const MAX_CHUNK_SIZE: u64 = 256 * 1024;

pub struct TransactionOffset {
    pub size: u64,
    pub offset: u64,
}

#[derive(Deserialize)]
struct JsonTransactionOffset {
    pub size: String,
    pub offset: String,
}

#[derive(Deserialize)]
struct JsonChunk {
    pub chunk: String,
}

// Per Hyper docs:
// `Client` is cheap to clone and cloning is the recommended way to share a `Client`. The underlying connection pool will be reused.
//
// So call this once. Then clone the result as necessary.
pub async fn get_client() -> Result<Client> {
    let https = HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    Ok(client)
}

pub async fn get_tx_offsets(client: &Client, tx_id: &str) -> Result<Vec<u64>> {
    let uri = format!("{}/tx/{}/offset", BASE_URI, tx_id);
    let res = request(client, &uri).await?;

    let body = hyper::body::aggregate(res).await?;
    let json_offset: JsonTransactionOffset = serde_json::from_reader(body.reader())?;

    let size: u64 = json_offset.size.parse()?;
    let offset: u64 = json_offset.offset.parse()?;

    Ok(calculate_offsets(size, offset))
}

pub async fn read_chunk(client: Client, offset: u64) -> Result<Vec<u8>> {
    let uri = format!("{}/chunk/{}", BASE_URI, offset);
    let res = request(&client, &uri).await?;

    let body = hyper::body::aggregate(res).await?;
    let json_chunk: JsonChunk = serde_json::from_reader(body.reader())?;

    let config = base64::Config::new(base64::CharacterSet::UrlSafe, false);
    Ok(base64::decode_config(json_chunk.chunk, config)?)
}

async fn request(client: &Client, uri: &str) -> Result<Response<Body>> {
    let uri = uri.parse()?;
    let res = client.get(uri).await?;

    let status = res.status();
    if status != 200 {
        bail!("Response: {}", status);
    }

    Ok(res)
}

fn calculate_offsets(size: u64, last_offset: u64) -> Vec<u64> {
    let mut offsets = vec![];

    let mut last_offset_len = size % MAX_CHUNK_SIZE;
    if last_offset_len == 0 {
        last_offset_len = MAX_CHUNK_SIZE;
    }

    let remaining = size - last_offset_len;

    for i in 0..remaining / MAX_CHUNK_SIZE {
        offsets.push(last_offset - last_offset_len - remaining + MAX_CHUNK_SIZE * (i + 1));
    }

    offsets.push(last_offset);

    offsets
}

#[cfg(test)]
mod tests {
    use crate::reader::calculate_offsets;

    #[test]
    fn first_offset() {
        let offsets = calculate_offsets(16492364, 106448066463810);

        assert_eq!(offsets[0], 106448050233590,);
    }

    #[test]
    fn last_offset() {
        let offsets = calculate_offsets(16492364, 106448066463810);

        assert_eq!(offsets.last(), Some(&106448066463810),);
    }

    #[test]
    fn num_offsets() {
        let offsets = calculate_offsets(16492364, 106448066463810);

        assert_eq!(offsets.len(), 63,);
    }
}
