use axum::extract::{Extension, Path};
use axum::response::Json;
use axum::routing::get;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::{ConnectOptions, FromRow};
use std::str::FromStr;
use std::sync::Arc;

/// Database object encapsulating the connection pool and providing convenience functions.
struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
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

        Ok(Self { pool })
    }
}

#[derive(FromRow, Serialize, Deserialize)]
struct Post {
    title: String,
    content: String,
}

async fn get_post(Path(id): Path<i64>, Extension(db): Extension<Arc<Database>>) -> Json<Post> {
    Json(
        sqlx::query_as::<_, Post>("SELECT title, content FROM posts WHERE id=?")
            .bind(id)
            .fetch_one(&db.pool)
            .await
            .unwrap(),
    )
}

async fn add_post(Extension(db): Extension<Arc<Database>>, Json(post): Json<Post>) {
    sqlx::query("INSERT INTO posts (title, content) VALUES (?, ?);")
        .bind(post.title)
        .bind(post.content)
        .execute(&db.pool)
        .await
        .unwrap();
}

async fn posts(Extension(db): Extension<Arc<Database>>) -> Json<Vec<Post>> {
    Json(
        sqlx::query_as::<_, Post>("SELECT title, content FROM posts")
            .fetch_all(&db.pool)
            .await
            .unwrap(),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(Database::new().await?);

    let app = axum::Router::new()
        .route("/api/posts", get(posts).post(add_post))
        .route("/api/posts/:id", get(get_post))
        .layer(Extension(state));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
