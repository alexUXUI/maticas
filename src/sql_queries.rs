use crate::models::{Plant};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_all_plants(client: &Client) -> Result<Vec<Plant>, io::Error> {
    let statement = client.prepare("select * from plant").await.unwrap();

    let plants = client
        .query(&statement, &[])
        .await
        .expect("Could not get results from all plants query")
        .iter()
        .map(|row| Plant::from_row_ref(row).unwrap())
        .collect::<Vec<Plant>>();
        
    Ok(plants)
}