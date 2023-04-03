mod api;
mod model;
mod repository;

use api::transactions;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(transactions::get_transactions)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
