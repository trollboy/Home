// main.rs
use warp::Filter;
use std::sync::Arc;
use juniper::http::graphiql::graphiql_source;

#[tokio::main]
async fn main () {
    let schema = Arc::new(Schema::new(QueryRoot, MutationRoot));
    // Create a warp filter for the schema
    let schema = warp::any().map(move || Arc::clone(&schema));

    let ctx = Arc::new(Context { client });
    // Create a warp filter for the context
    let ctx = warp::any().map(move || Arc::clone(&ctx));

    let graphql_route = warp::post()
        .and(warp::path!("graphql"))
        .and(schema.clone())
        .and(ctx.clone())
        .and(warp::body::json())
        .and_then(graphql);

    let graphiql_route = warp::get()
        .and(warp::path!("graphiql"))
        .map(|| warp::reply::html(graphiql_source("graphql")));

    let routes = graphql_route.or(graphiql_route);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
