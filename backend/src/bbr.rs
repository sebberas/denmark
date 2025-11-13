use graphql_client::GraphQLQuery;
use reqwest::StatusCode;
use std::env;
pub type Long = i64;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/BBR.graphql",
    query_path = "queries/BBR_relations/ejendomrelation.query.graphql",
    response_derives = "Debug",
)]
pub struct BbrEjendomRelationQuery;

pub async fn get_ejendom_relations() -> anyhow::Result<Vec<BbrEjendomRelationQuery>> {
    let client = reqwest::Client::builder().build().unwrap();

    let variables = bbr_ejendom_relation_query::Variables {};

    let request = client
        .post(format!(
            "https://graphql.datafordeler.dk/BBR/v1?apikey={}",
            env::var("DATAFORDELER_API_KEY")?
        ))
        .json(&serde_json::to_value(BbrEjendomRelationQuery::build_query(variables)).unwrap());

    let response = request.send().await?;

    match response.status() {
        StatusCode::OK => {
            let body = response.json::<bbr_ejendom_relation_query::ResponseData>().await?;

            tracing::info!(body = ?body, "Response Status, bbr ejendome relation query");
        },
        _ => {
            tracing::info!(status = ?response.status(), "Response Status, bbr ejendome relation query");
            tracing::info!(text = ?response.text().await, "Response Text, bbr ejendome relation query");
        }
    }

    todo!()
}
