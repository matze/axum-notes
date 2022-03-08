use axum::body::BoxBody;
use axum::http::{header, Request, StatusCode};
use axum::response::Response;
use axum::routing::get;
use axum::{Router, Server};
use tower_http::auth::{AuthorizeRequest, RequireAuthorizationLayer};

#[derive(Clone)]
struct Auth {
    expected: String,
}

impl<B> AuthorizeRequest<B> for Auth {
    type ResponseBody = BoxBody;

    fn authorize(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        if let Some(header) = request.headers().get(header::AUTHORIZATION) {
            if header == &self.expected {
                return Ok(());
            }
        }

        let response = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(BoxBody::default())
            .unwrap();

        Err(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(|| async { "hello" })).layer(
        RequireAuthorizationLayer::custom(Auth {
            expected: "lol".to_string(),
        }),
    );

    Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
