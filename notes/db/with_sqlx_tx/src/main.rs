use axum::extract::Path;
use axum::response::Json;
use axum::routing::get;
use axum_sqlx_tx::Tx;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{Sqlite, SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::{ConnectOptions, FromRow};
use std::str::FromStr;

async fn new_pool() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let db_options = SqliteConnectOptions::from_str(&":memory:")?
        .create_if_missing(true)
        .disable_statement_logging()
        .to_owned();

    let pool = SqlitePoolOptions::new().connect_with(db_options).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            content NOT NULL
        );",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

#[derive(FromRow, Serialize, Deserialize)]
struct Post {
    title: String,
    content: String,
}

async fn get_post(Path(id): Path<i64>, mut tx: Tx<Sqlite>) -> Json<Post> {
    Json(
        sqlx::query_as::<_, Post>("SELECT title, content FROM posts WHERE id=?")
            .bind(id)
            .fetch_one(&mut tx)
            .await
            .unwrap(),
    )
}

async fn add_post(mut tx: Tx<Sqlite>, Json(post): Json<Post>) {
    sqlx::query("INSERT INTO posts (title, content) VALUES (?, ?);")
        .bind(post.title)
        .bind(post.content)
        .execute(&mut tx)
        .await
        .unwrap();
}

async fn posts(mut tx: Tx<Sqlite>) -> Json<Vec<Post>> {
    Json(
        sqlx::query_as::<_, Post>("SELECT title, content FROM posts")
            .fetch_all(&mut tx)
            .await
            .unwrap(),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = axum::Router::new()
        .route("/api/posts", get(posts).post(add_post))
        .route("/api/posts/:id", get(get_post))
        .layer(axum_sqlx_tx::Layer::new(new_pool().await?));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
