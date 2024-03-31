// src/db.rs
use sqlx::{PgPool, Error};

pub async fn connect_to_database(url: &str) -> Result<PgPool, Error> {
    let pool = PgPool::connect(url).await?;
    Ok(pool)
}

pub async fn query_database(pool: &PgPool) -> Result<(), Error> {
    // Example query
    // let rows = sqlx::query!("SELECT * FROM my_table")
    //     .fetch_all(pool)
    //     .await?;

    // println!("Query results: {:?}", rows);
    
    Ok(())
}
