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
    pub cvr_enheds_id: String,
    pub datafordeler_opdateringstid: DateTime<Utc>,
    pub feltliste: BrancheFelt,
    pub registrering_fra: DateTime<Utc>,
    pub registrering_til: Option<DateTime<Utc>>,
    pub registreringsaktoer: String,
    pub sekvens: u64,
    pub vaerdi: String,
    pub vaerdi_tekst: String,
    pub virkning_fra: NaiveDate,
    pub virkning_til: Option<NaiveDate>,
    pub virkningsaktoer: String,
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
    pub cvr_enheds_id: String,
    pub datafordeler_opdateringstid: DateTime<Utc>,
    pub feltliste: NavnFelt,
    pub registrering_fra: DateTime<Utc>,
    pub registrering_til: Option<DateTime<Utc>>,
    pub registreringsaktoer: String,
    pub sekvens: u64,
    pub vaerdi: String,
    pub virkning_fra: NaiveDate,
    pub virkning_til: Option<NaiveDate>,
    pub virkningsaktoer: String,
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
    Inaktiv,
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
    CvrEnhedsId,
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
pub enum VirksomhedsFeltliste {
    Aendret,
    Oprettet,
    Ophoert,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VirksomhedsStatus {
    Aktiv,
    Inaktiv,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VirksomhedRecord {
    #[serde(rename = "CVRNummer")]
    pub cvr_nummer: NonZeroU64,
    pub datafordeler_opdateringstid: Option<DateTime<Utc>>,
    pub feltliste: VirksomhedsFeltliste,
    pub id: String,
    pub registrering_fra: DateTime<Utc>,
    pub registrering_til: Option<DateTime<Utc>>,
    pub status: VirksomhedsStatus,
    pub virkning_fra: NaiveDate,
    pub virkning_til: Option<NaiveDate>,
    pub virkningsaktoer: String,
    pub virksomhed_ophoersdato: Option<NaiveDate>,
    pub virksomhed_startdato: NaiveDate,
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

#[derive(Debug, Deserialize, Serialize)]
pub enum ProduktionsenhedFelt {
    Oprettet,
    Aendret,
    Ophoert,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProduktionsenhedStatus {
    Aktiv,
    Inaktiv,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProduktionsenhedRecord {
    pub datafordeler_opdateringstid: Option<DateTime<Utc>>,
    pub feltliste: ProduktionsenhedFelt,
    pub id: String,
    pub p_nummer: NonZeroU64,
    pub produktionsenhed_ophoersdato: Option<NaiveDate>,
    pub produktionsenhed_startdato: NaiveDate,
    pub registrering_fra: DateTime<Utc>,
    pub registrering_til: Option<DateTime<Utc>>,
    pub registreringsaktoer: String,
    pub status: ProduktionsenhedStatus,
    pub tilknyttet_til_virksomhed_ophoersdato: Option<NaiveDate>,
    pub tilknyttet_til_virksomhed_startdato: NaiveDate,
    #[serde(rename = "tilknyttetVirksomhedsCVRNummer")]
    pub tilknyttet_virksomheds_cvr_nummer: NonZeroU64,
    pub virkning_fra: NaiveDate,
    pub virkning_til: Option<NaiveDate>,
    pub virkningsaktoer: String,
}

#[tracing::instrument(skip(client))]
pub async fn download_produktionsenhed_list(
    client: &Client,
) -> anyhow::Result<Vec<ProduktionsenhedRecord>> {
    let contents = download_file(
        client,
        "CVR_V1_Produktionsenhed_TotalDownload_json_Current_193.zip",
    )
    .await?;
    Ok(serde_json::from_value(contents)?)
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
