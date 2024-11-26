use sqlx::{migrate::Migrator, PgPool};
use std::env;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    MIGRATOR.run(&pool).await?;
    println!("Migrations applied successfully");
    Ok(())
}