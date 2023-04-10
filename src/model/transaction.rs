use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    pub id: i32,
    pub value: f64,
    pub business_name: String,
    pub category: String,
}
#[derive(Deserialize, Serialize)]
pub struct NewTransaction {
    pub value: f64,
    pub business_name: String,
    pub category: String,
}

pub fn map_row_to_transaction(row: &Row) -> Transaction {
    Transaction {
            id: row.get(0),
            value: row.get(1),
            business_name: row.get(2),
            category: row.get(3),
    }
}

