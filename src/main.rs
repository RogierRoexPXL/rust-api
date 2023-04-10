mod api;
mod model;
mod repository;

use api::transactions;
use api::helloworld;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_module("icountant_api", log::LevelFilter::Debug)
        .init();

    // Get the address to bind to from environment variables or default to 127.0.0.1:8080
    let bind_address = env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_string());

    // Start the server
    HttpServer::new(|| {
        App::new()
            .service(transactions::get_transactions)
            .service(transactions::create_transaction)
            .service(helloworld::hello)
    })
    .bind(bind_address)?
    .run()
    .await
}
