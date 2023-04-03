use postgres::{Client, Error, NoTls};

pub async fn create_client(database_url: &str) -> Client {
    mut client = Client::connect(
        database_url, NoTls
    )?;
}
