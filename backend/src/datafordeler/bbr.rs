use super::download_file;
use reqwest::Client;
use ::serde_json::{Value as JsonValue};

#[tracing::instrument(skip(client))]
pub async fn download_ejendom_relation(client: &Client) -> anyhow::Result<JsonValue> {
    download_file(client, "BBR_V2_Ejendomsrelation_TotalDownload_json_Current_445.zip").await
}



#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_branche_list() {
        let _ = dotenvy::dotenv();

        let client = Client::new();

        let list = download_ejendom_relation(&client).await.unwrap();
    }
}