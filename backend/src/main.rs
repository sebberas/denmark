pub mod telemetry;
pub mod bbr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    telemetry::init();

    tracing::info!("hello world!");

    Ok(())
}