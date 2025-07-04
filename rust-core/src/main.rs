use std::error::Error;
use sqlx::{migrate::MigrateDatabase, Sqlite};

const DB_URL: &str = "sqlite:./sqlite/sqlite.db";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    if!Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {//Creates the sqlite.db file if it's not present
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }        
    }

    let pool = sqlx::sqlite::SqlitePool::connect(DB_URL).await?;
    sqlx::migrate!("./migrations")//From the database pool connection, it would use the schema file in the migrations directory to create the db tables
        .run(&pool)
        .await?;
    
    Ok(())
}