#[macro_use]
extern crate nanoid;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_repr;

use tokio::net::TcpListener;

mod database;
mod models;
mod routers;

use database::Database;

const ADDR: &str = "localhost:3000";
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result {
    dotenv::dotenv().unwrap();

    let url = std::env::var("DATABASE_URL").expect("There must be a DATABASE_URL");
    let database = Database::connect(&url).await?;

    run_server(database).await
}

async fn run_server(database: Database) -> Result {
    let app = routers::app(database);
    let listener = TcpListener::bind(ADDR).await?;

    println!("The server is listening on {ADDR}");

    axum::serve(listener, app).await?;

    Ok(())
}
