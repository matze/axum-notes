use askama::Template;
use axum::extract::Path;
use axum::routing::get;
use axum::Server;

#[derive(Template)]
#[template(path = "index.html")]
struct HtmlTemplate {
    name: String,
    age: u8,
}

async fn index(Path((name, age)): Path<(String, u8)>) -> HtmlTemplate {
    HtmlTemplate { name, age }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = axum::Router::new().route("/:name/:age", get(index));

    Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
