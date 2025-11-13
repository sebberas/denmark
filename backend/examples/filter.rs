use std::collections::HashMap;

use ::serde::Deserialize;
use ::tokio::fs;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    enheds_type: String,
    forretningsnoegletype: String,
    status: String,
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let contents = fs::read("data/enhed.json").await?;

    let mut map = HashMap::new();

    let records: Vec<Record> = serde_json::from_slice(&contents)?;
    for record in records {
        let field = record.forretningsnoegletype;

        if let Some(n) = map.get_mut(&field) {
            *n += 1
        } else {
            map.insert(field, 1);
        }
    }

    println!("{:#?}", map);


    Ok(())
    
}