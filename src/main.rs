#[macro_use]
extern crate nanoid;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_repr;

use tokio::net::TcpListener;

mod database;
mod error;
mod models;
mod routers;

use database::Database;
use error::Result;

#[tokio::main]
async fn main() -> Result {
    dotenv::dotenv().unwrap();

    let url = std::env::var("DATABASE_URL").expect("There must be a DATABASE_URL");
    let database = Database::connect(&url).await?;

    run_server(database).await
}

async fn run_server(database: Database) -> Result {
    let app = routers::app(database);
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    if let Ok(addr) = listener.local_addr() {
        println!("The server is listening on {addr}");
    }

    axum::serve(listener, app).await?;

    Ok(())
}
