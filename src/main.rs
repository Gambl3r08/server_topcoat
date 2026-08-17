use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{Router, RouterBuilderDiscoverExt, page, parse_query_params},
    view::{component, view},
};

#[tokio::main]
async fn main() {
    topcoat::start(Router::builder().discover().build())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct IndexQuery {
    name: Option<String>,
}

#[page("/")]
async fn index(cx: &Cx) -> Result {
    let query: IndexQuery = parse_query_params(cx)?;
    let query_name = query.name.as_deref().unwrap_or("World");

    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Hello, world"</title>
                topcoat::dev::script()
            </head>
            <body>
                hello(name: query_name)
            </body>
        </html>
    }
}

#[component]
async fn hello(name: &str) -> Result {
    view! {
        <h1>"Hello, " (name) "!"</h1>
    }
}
