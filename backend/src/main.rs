pub mod telemetry;
pub mod bbr;
pub mod cvr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    telemetry::init();

    bbr::get_ejendom_relations().await?;

    Ok(())
}