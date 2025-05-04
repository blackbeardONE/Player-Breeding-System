use sqlx::{MySql, Pool};
use anyhow::Result;
use sqlx::migrate::Migrator;
use std::path::Path;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

use tokio::time::{sleep, Duration};

pub async fn init_db(database_url: &str) -> Result<Pool<MySql>> {
    let mut attempts = 0;
    let pool;
    loop {
        match Pool::<MySql>::connect(database_url).await {
            Ok(p) => {
                pool = p;
                break;
            }
            Err(e) => {
                attempts += 1;
                if attempts >= 10 {
                    return Err(e.into());
                }
                eprintln!("Database connection failed (attempt {}): {}. Retrying in 3 seconds...", attempts, e);
                sleep(Duration::from_secs(3)).await;
            }
        }
    }
    MIGRATOR.run(&pool).await?;
    Ok(pool)
}
