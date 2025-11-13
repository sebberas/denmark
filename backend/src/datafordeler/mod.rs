use std::env;
use std::io::{Cursor};

use ::serde_json::{Value as JsonValue};
use ::anyhow::anyhow;
use ::reqwest::Client;
use ::reqwest::StatusCode;

pub mod cvr;

const BASE_URL: &str = "https://api.datafordeler.dk/FileDownloads/GetFile";


#[tracing::instrument(skip(client))]
pub async fn download_file(client: &Client, filename: &str) -> anyhow::Result<JsonValue> {
    let url = format!("{BASE_URL}?Filename={filename}&apikey={}", env::var("DATAFORDELER_API_KEY")?);
    let request = client.get(url);
    let response = request.send().await?;

    match response.status() {
        StatusCode::OK => {
            // TODO: Check if zip?
            let body = response.bytes().await?;
            let mut archive = zip::ZipArchive::new(Cursor::new(body))?;
            let entry = archive.by_index(0)?;
            let contents = serde_json::from_reader::<_, JsonValue>(entry)?;            
            Ok(contents)
        }
        status => {
            let text = response.text().await;
            Err(anyhow!("failed to download file {status}\n{text:?}"))
        }
    }
}