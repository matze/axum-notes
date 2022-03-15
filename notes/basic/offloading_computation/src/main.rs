use axum::extract::{Extension, Path};
use axum::response::Json;
use axum::routing::get;
use tokio::sync::{mpsc, oneshot};

// Commands send to the processor. We use the oneshot sender to "call back" to the peer who send us
// the command.
enum Command {
    Sleep {
        secs: u64,
        // Using oneshot::Sender<Result<_>> we can propagate errors back to the caller
        tx: oneshot::Sender<Result<u64, ()>>,
    },
}

async fn process_compute_request(mut rx: mpsc::Receiver<Command>) {
    // Note, that this serializes incoming requests. If you want to handle requests in parallel,
    // you have to spawn tasks once more.
    while let Some(command) = rx.recv().await {
        match command {
            Command::Sleep { secs, tx } => {
                let _ = tx.send(Ok(secs + 23));
            }
        }
    }
}

async fn compute_complex(
    Path(secs): Path<u64>,
    Extension(command_tx): Extension<mpsc::Sender<Command>>,
) -> Json<u64> {
    // Construct a oneshot channel to receive the result from the processor.
    let (tx, rx) = oneshot::channel();

    // Send the command carrying the payload as well as the result sender.
    let _ = command_tx.send(Command::Sleep { secs, tx }).await;

    // Wait for the result to be returned by the processor.
    let result = rx.await.unwrap().unwrap();

    Json(result)
}

async fn compute_simple(Path(secs): Path<u64>) -> Json<u64> {
    println!("asked to sleep for {secs} secs");

    // Spawn an async task on a separate thread to avoid blocking the async run-time.
    let result = tokio::task::spawn_blocking(move || {
        // Unlike tokio::time::sleep, this one blocks the current thread.
        std::thread::sleep(std::time::Duration::from_secs(secs));
        secs + 42
    })
    .await
    .unwrap();

    println!("returned after {secs} secs");

    Json(result)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel(128);

    let app = axum::Router::new()
        .route("/compute/simple/:secs", get(compute_simple))
        .route("/compute/complex/:secs", get(compute_complex))
        .layer(Extension(tx));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    let server = tokio::task::spawn(async move {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    let processor = tokio::task::spawn(async move {
        process_compute_request(rx).await;
    });

    let (_, _) = tokio::join!(server, processor);

    Ok(())
}
