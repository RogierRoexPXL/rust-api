mod api;
mod model;
mod repository;

use api::transactions;
use api::helloworld;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(transactions::get_transactions)
            .service(helloworld::hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
