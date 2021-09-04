use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "files/schema.docs.graphql",
    query_path = "files/get_repositories_from_user.graphql",
    response_derives = "Debug",
    module_visibility = "pub"
)]
pub struct GetRepositoriesFromUser;
