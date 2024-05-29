use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

mod database;
mod download;
mod upload;

use database::Database;

const ADDR: &str = "localhost:3000";
const BODY_LIMIT: usize = 1024 * 1_000_000;
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result {
    dotenv::dotenv().unwrap();

    let url = std::env::var("DATABASE_URL").expect("There must be a DATABASE_URL");
    let database = Database::connect(&url).await?;

    run_server(database).await
}

async fn run_server(database: Database) -> Result {
    let app = Router::new()
        .route("/", post(upload::handle))
        .layer(DefaultBodyLimit::max(BODY_LIMIT))
        .route("/:name", get(download::handle))
        .with_state(database);
    let listener = TcpListener::bind(ADDR).await?;

    println!("The server is listening on {ADDR}");

    axum::serve(listener, app).await?;

    Ok(())
}
