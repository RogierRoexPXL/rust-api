use crate::model::transaction::Transaction;
use crate::repository::postgresdb::establish_connection;

use actix_web::{get, Responder, HttpResponse, post, web};
use serde_json::to_string;

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

#[post("/transactions")]
async fn create_transaction(transaction: web::Json<Transaction>) -> impl Responder {
    let client = establish_connection().await.unwrap();
    let rows = client
    .execute(
        "INSERT INTO transactions (value, business_name, category) VALUES ($1, $2, $3)",
        &[&transaction.value, &transaction.business_name, &transaction.category],
    )
    .await
    .expect("error executing query");

    if rows == 1 {
        HttpResponse::Created().json(transaction.into_inner())
    } else {
        HttpResponse::InternalServerError().finish()
    }
}