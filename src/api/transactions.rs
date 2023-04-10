use crate::model::transaction::{Transaction, NewTransaction, map_row_to_transaction};
use crate::repository::postgresdb::establish_connection;

use actix_web::{get, Responder, HttpResponse, post, web, put};
use log::info;
use serde_json::to_string;

#[get("/transactions")]
async fn get_transactions() -> impl Responder {
    let client = establish_connection().await.unwrap();
    let rows = client
        .query("SELECT * FROM transactions", &[])
        .await
        .unwrap();
    let transactions: Vec<Transaction> = rows
        .iter()
        .map(map_row_to_transaction)
        .collect();
    HttpResponse::Ok().body(to_string(&transactions).unwrap())
}

#[get("/transactions/{id}")]
async fn get_transaction_by_id(id: web::Path<i32>) -> impl Responder {
    let client = establish_connection().await.unwrap();
    let id_value = id.into_inner();
    info!("Fetching transaction with ID: {}", id_value);

    let row = client
        .query_opt("SELECT * FROM transactions WHERE id = $1", &[&id_value])
        .await
        .unwrap();
    
    //match does conditional branching. row = Option<Row> with 2 possible outcomes: Some(row) || None 
    match row {
        Some(row) => {
            let transaction = map_row_to_transaction(&row);
            HttpResponse::Ok().body(to_string(&transaction).unwrap())
        }
        None => HttpResponse::NotFound().finish()
    }
}


#[post("/transactions")]
async fn create_transaction(transaction: web::Json<NewTransaction>) -> impl Responder {
    let client = establish_connection().await.unwrap();
    let new_transaction = transaction.into_inner();

    let row = client
    .query_one(
        "INSERT INTO transactions (value, business_name, category) 
        VALUES ($1, $2, $3) 
        RETURNING id, value, business_name, category",
        &[&new_transaction.value, &new_transaction.business_name, &new_transaction.category],
    )
    .await;

    match row {
        Ok(row) => {
            let created_transaction = map_row_to_transaction(&row);
            HttpResponse::Created().json(created_transaction)
        }
        Err(_) => HttpResponse::InternalServerError().finish()
    }
    
}

#[put("/transactions/{id}")]
async fn update_transaction(id: web::Path<i32>, transaction: web::Json<NewTransaction>) -> impl Responder {
    let client = establish_connection().await.unwrap();
    let id_value = id.into_inner();

    let row = client
    .query_one(
        "UPDATE transactions 
        SET value = $2,
            business_name = $3,
            category = $4
        WHERE id = $1 
        RETURNING id, value, business_name, category;",
        &[&id_value, &transaction.value, &transaction.business_name, &transaction.category],
    )
    .await;

    match row {
        Ok(row) => {
            let updated_transaction = map_row_to_transaction(&row);
            HttpResponse::Created().json(updated_transaction)
        }
        Err(_) => HttpResponse::InternalServerError().finish()
    }
    
}

