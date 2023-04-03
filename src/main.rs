mod model;

use model::transaction::Transaction;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::to_string;
use tokio_postgres::{NoTls, Error};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_transactions)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/transactions")]
async fn get_transactions() -> impl Responder {
    let client = establish_connection().await.unwrap();
    let rows = client
        .query("SELECT id, value, business_name, category FROM transactions", &[])
        .await
        .unwrap();
    let transactions: Vec<Transaction> = rows
        .iter()
        .map(|row| Transaction {
            id: row.get(0),
            value: row.get(1),
            business_name: row.get(2),
            category: row.get(3),
        })
        .collect();
    HttpResponse::Ok().body(to_string(&transactions).unwrap())
}

async fn establish_connection() -> Result<tokio_postgres::Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=127.0.0.1 port=5432 dbname=transaction_db user=postgres password=pgdbRogier22",
        NoTls,
    )
    .await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}