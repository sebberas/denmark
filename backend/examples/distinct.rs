use std::collections::HashMap;

use ::serde::Deserialize;
use ::tokio::fs;


#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CvrEnhedStatus {
    Aktiv,
    Inaktiv
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum CvrFelt {
    Oprettet,
    Aendret,
    Ophoert,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    /// unique identification
    id: String,
    enheds_type: String,
    forretningsnoegle: String,
    forretningsnoegletype: String,
    feltliste: CvrFelt,
    status: CvrEnhedStatus,
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let contents = fs::read("data/enhed.json").await?;

    let mut map = HashMap::new();

    let records: Vec<Record> = serde_json::from_slice(&contents)?;
    for record in records {
        
        let field = (record.status, record.feltliste, record.enheds_type);

        if let Some(n) = map.get_mut(&field) {
            *n += 1
        } else {
            map.insert(field, 1);
        }
    }

    println!("{:#?}", map);


    Ok(())
    
}