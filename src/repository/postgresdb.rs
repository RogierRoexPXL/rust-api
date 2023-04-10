use dotenv::dotenv;
use std::env;
use tokio_postgres::{NoTls, Error};
use log::{error, info};

pub async fn establish_connection() -> Result<tokio_postgres::Client, Error> {
    
    // Get the database connection details from environment variables
    dotenv().ok();
    let host = env::var("PGHOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PGPORT").unwrap_or_else(|_| "5432".to_string());
    let database = env::var("PGDATABASE").unwrap_or_else(|_| "postgres".to_string());
    let user = env::var("PGUSER").unwrap_or_else(|_| "postgres".to_string());
    let password = env::var("PGPASSWORD").unwrap_or_else(|_| "password".to_string());


    // Configure the database connection
    let conn_string = format!(
        "host={} port={} dbname={} user={} password={}",
        host, port, database, user, password
    );
    let (client, connection) = tokio_postgres::connect(&conn_string, NoTls).await?;

    // Log the successful connection
    info!("Connected to database: {}", conn_string);

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Connection error: {} with username {} and password {}", e, user, password);
        }
    });

    Ok(client)
}
