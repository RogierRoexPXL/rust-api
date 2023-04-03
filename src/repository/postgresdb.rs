use tokio_postgres::{NoTls, Error};

pub async fn establish_connection() -> Result<tokio_postgres::Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost port=5432 dbname=transaction_db user=postgres password=pgdbRogier22",
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