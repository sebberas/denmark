use graphql_client::*;

pub type Long = i64;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/BBR.graphql",
    query_path = "queries/BBR_relations/ejendomrelation.query.graphql",
)]
struct BbrEjendomRelationQuery;

