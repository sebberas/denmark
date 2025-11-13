use ::reqwest::Client;
use ::tokio::fs;

pub use datafordeler::*;

pub mod telemetry;
pub mod bbr;
pub mod datafordeler;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    telemetry::init();

    let client = Client::new();

    let list = cvr::download_branche_list(&client).await?;

    fs::write("list.json", serde_json::to_string_pretty(&list)?).await?;

    Ok(())
}