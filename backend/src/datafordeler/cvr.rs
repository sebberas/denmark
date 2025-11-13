use ::serde_json::{Value as JsonValue};
use ::reqwest::Client;

use super::download_file;

#[tracing::instrument(skip(client))]
pub async fn download_branche_list(client: &Client) -> anyhow::Result<JsonValue> {
    download_file(client, "CVR_V1_Branche_TotalDownload_json_Current_193.zip").await
}

#[tracing::instrument(skip(client))]
pub async fn download_navn_list(client: &Client) -> anyhow::Result<JsonValue> {
    download_file(client, "CVR_V1_Navn_TotalDownload_json_Current_193.zip").await
}

#[tracing::instrument(skip(client))]
pub async fn download_enhed_list(client: &Client) -> anyhow::Result<JsonValue> {
    download_file(client, "CVR_V1_CVREnhed_TotalDownload_json_Bitemporal_193.zip").await
}

#[tracing::instrument(skip(client))]
pub async fn download_virksomhed_list(client: &Client) -> anyhow::Result<JsonValue> {
    download_file(client, "CVR_V1_Virksomhed_TotalDownload_json_Current_193.zip").await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_branche_list() {
        let _ = dotenvy::dotenv();

        let client = Client::new();

        let list = download_branche_list(&client).await.unwrap();
    }
}