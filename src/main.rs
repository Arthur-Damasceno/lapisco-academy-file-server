use axum::Router;
use std::error::Error;
use tokio::net::TcpListener;

const ADDR: &str = "localhost:3000";

#[tokio::main]
async fn main() {
    if let Err(err) = run_server().await {
        eprintln!("There was an error: {err}");
    }
}

async fn run_server() -> Result<(), Box<dyn Error>> {
    let app = Router::new();
    let listener = TcpListener::bind(ADDR).await?;

    println!("The server is listening on {ADDR}");

    axum::serve(listener, app).await?;

    Ok(())
}
