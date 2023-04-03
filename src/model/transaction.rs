use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    pub id: i32,
    pub value: f64,
    pub business_name: String,
    pub category: String,
}