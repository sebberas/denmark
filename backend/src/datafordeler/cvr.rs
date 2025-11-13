use ::chrono::{DateTime, NaiveDate, Utc};
use ::reqwest::Client;
use ::serde::{Deserialize, Serialize};
use ::serde_json::Value as JsonValue;
use ::std::num::NonZeroU64;

use super::download_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum BrancheFelt {
    Oprettet,
    Aendret,
    Ophoert,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BrancheRecord {
    #[serde(rename = "CVREnhedsId")]
    cvr_enheds_id: String,
    datafordeler_opdateringstid: DateTime<Utc>,
    feltliste: BrancheFelt,
    registrering_fra: DateTime<Utc>,
    registrering_til: Option<DateTime<Utc>>,
    registreringsaktoer: String,
    sekvens: u64,
    vaerdi: String,
    vaerdi_tekst: String,
    virkning_fra: DateTime<Utc>,
    virkning_til: Option<DateTime<Utc>>,
    virkningsaktoer: String,
}

#[tracing::instrument(skip(client))]
pub async fn download_branche_list(client: &Client) -> anyhow::Result<Vec<BrancheRecord>> {
    let contents =
        download_file(client, "CVR_V1_Branche_TotalDownload_json_Current_193.zip").await?;
    Ok(serde_json::from_value(contents)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum NavnFelt {
    Oprettet,
    Aendret,
    Ophoert,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NavnRecord {
    #[serde(rename = "CVREnhedsId")]
    cvr_enheds_id: String,
    datafordeler_opdateringstid: DateTime<Utc>,
    feltliste: NavnFelt,
    registrering_fra: DateTime<Utc>,
    registrering_til: Option<DateTime<Utc>>,
    registreringsaktoer: String,
    sekvens: u64,
    vaerdi: String,
    virkning_fra: DateTime<Utc>,
    virkning_til: Option<DateTime<Utc>>,
    virkningsaktoer: String,
}

#[tracing::instrument(skip(client))]
pub async fn download_navn_list(client: &Client) -> anyhow::Result<Vec<NavnRecord>> {
    let contents = download_file(client, "CVR_V1_Navn_TotalDownload_json_Current_193.zip").await?;
    Ok(serde_json::from_value(contents)?)
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EnhedStatus {
    Aktiv,
    Inaktiv
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub enum EnhedFelt {
    Oprettet,
    Aendret,
    Ophoert,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub enum EnhedsType {
    Virksomhed,
    Produktionsenhed,
    #[serde(rename = "CVRPerson")]
    CvrPerson,
    AndreDeltagerePerson,
    AndreDeltagereVirksomhed,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub enum EnhedForretningsnoegletype {
    #[serde(rename = "CVRNummer")]
    CvrNummer,
    #[serde(rename = "pNummer")]
    PNummer,
    #[serde(rename = "CVREnhedsId")]
    CvrEnhedsId
} 

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnhedRecord {
    pub id: String,
    pub enheds_type: EnhedsType,
    pub feltliste: EnhedFelt,
    pub forretningsnoegle: String,
    pub forretningsnoegletype: EnhedForretningsnoegletype,
    pub status: EnhedStatus,
}

#[tracing::instrument(skip(client))]
pub async fn download_enhed_list(client: &Client) -> anyhow::Result<Vec<EnhedRecord>> {
    let contents = download_file(
        client,
        "CVR_V1_CVREnhed_TotalDownload_json_Bitemporal_193.zip",
    )
    .await?;

    Ok(serde_json::from_value(contents)?)
}

#[derive(Debug, Deserialize, Serialize)]
enum VirksomhedsFeltliste {
    Aendret,
    Oprettet,
    Ophoert,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum VirksomhedsStatus {
    Aktiv,
    Inaktiv,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VirksomhedRecord {
    #[serde(rename = "CVRNummer")]
    cvr_nummer: NonZeroU64,
    datafordeler_opdateringstid: Option<DateTime<Utc>>,
    feltliste: VirksomhedsFeltliste,
    id: String,
    registrering_fra: DateTime<Utc>,
    registrering_til: Option<DateTime<Utc>>,
    status: VirksomhedsStatus,
    virkning_fra: NaiveDate,
    virkning_til: Option<NaiveDate>,
    virkningsaktoer: String,
    virksomhed_ophoersdato: Option<NaiveDate>,
    virksomhed_startdato: NaiveDate,
}

#[tracing::instrument(skip(client))]
pub async fn download_virksomhed_list(client: &Client) -> anyhow::Result<Vec<VirksomhedRecord>> {
    let contents = download_file(
        client,
        "CVR_V1_Virksomhed_TotalDownload_json_Current_193.zip",
    )
    .await?;
    Ok(serde_json::from_value(contents)?)
}

#[tracing::instrument(skip(client))]
pub async fn download_produktionsenhed_list(client: &Client) -> anyhow::Result<JsonValue> {
    download_file(
        client,
        "CVR_V1_Produktionsenhed_TotalDownload_json_Current_193.zip",
    )
    .await
}

#[tracing::instrument(skip(client))]
pub async fn download_adressering_list(client: &Client) -> anyhow::Result<JsonValue> {
    download_file(
        client,
        "CVR_V1_Adressering_TotalDownload_json_Current_193.zip",
    )
    .await
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
