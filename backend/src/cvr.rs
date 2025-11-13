use std::env;

use anyhow::anyhow;
use ::reqwest::Client;
use ::reqwest::StatusCode;

const BASE_URL: &str = "https://api.datafordeler.dk/FileDownloads/GetFile?Filename=CVR_V1_Branche_TotalDownload_json_Current_193.zip";

#[tracing::instrument(skip(client))]
pub async fn get_branche_list(client: &Client) -> anyhow::Result<()> {
    let url = format!("{BASE_URL}&apikey={}", env::var("DATAFORDELER_API_KEY")?);
    let request = client.get(url);
    let response = request.send().await?;

    match response.status() {
        StatusCode::OK => {
            
            todo!()

        }
        status => {
            let text = response.text().await;
            Err(anyhow!("failed to download branche list {status}\n{text:?}"))
        }
    }
}