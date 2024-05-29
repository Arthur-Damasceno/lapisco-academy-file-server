use axum::{routing::get, Router};
use tokio::net::TcpListener;

mod download;
mod upload;

const ADDR: &str = "localhost:3000";
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[tokio::main]
async fn main() {
    if let Err(err) = run_server().await {
        eprintln!("There was an error: {err}");
    }
}

async fn run_server() -> Result {
    let app = Router::new().route("/", get(download::handle).post(upload::handle));
    let listener = TcpListener::bind(ADDR).await?;

    println!("The server is listening on {ADDR}");

    axum::serve(listener, app).await?;

    Ok(())
}
