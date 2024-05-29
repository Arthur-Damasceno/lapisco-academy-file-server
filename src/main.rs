use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

mod download;
mod upload;

const ADDR: &str = "localhost:3000";
const BODY_LIMIT: usize = 1024 * 1_000_000;
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[tokio::main]
async fn main() {
    if let Err(err) = run_server().await {
        eprintln!("There was an error: {err}");
    }
}

async fn run_server() -> Result {
    let app = Router::new()
        .route("/", post(upload::handle))
        .layer(DefaultBodyLimit::max(BODY_LIMIT))
        .route("/:name", get(download::handle));
    let listener = TcpListener::bind(ADDR).await?;

    println!("The server is listening on {ADDR}");

    axum::serve(listener, app).await?;

    Ok(())
}
