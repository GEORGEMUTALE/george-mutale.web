use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    println!("🚀 Starting database connection test...");

    // Get database URL from environment variable
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment variables or .env file");

    println!("📡 Connecting to: {}", mask_password(&database_url));

    // Create a connection pool with configuration
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&database_url)
        .await
        .map_err(|e| {
            format!(
                "❌ Failed to create connection pool: {}\n\
                 💡 Make sure:\n\
                 - Your database is running\n\
                 - The connection string is correct\n\
                 - You have internet connection (for Neon)\n\
                 - TLS features are enabled in Cargo.toml", 
                e
            )
        })?;

    println!("✅ Successfully created connection pool!");

    // Test the connection
    test_connection(&pool).await?;

    // Get and print PostgreSQL version
    match get_postgres_version(&pool).await {
        Ok(version) => println!("🐘 PostgreSQL Version: {}", version),
        Err(e) => println!("⚠️  Could not get version: {}", e),
    }

    println!("🎉 Database connection successful!");

    Ok(())
}

// Helper function to mask password in connection string for logging
fn mask_password(connection_string: &str) -> String {
    if let Some(at_index) = connection_string.find('@') {
        if let Some(colon_index) = connection_string[..at_index].find(':') {
            let mut masked = connection_string.to_string();
            let password_start = colon_index + 1;
            let password_end = at_index;
            if password_end > password_start {
                masked.replace_range(password_start..password_end, "***");
            }
            return masked;
        }
    }
    connection_string.to_string()
}

// Test the database connection
async fn test_connection(pool: &Pool<Postgres>) -> Result<()> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .map_err(|e| format!("Database connection test failed: {}", e))?;
    
    println!("✅ Database connection test passed!");
    Ok(())
}

// Get PostgreSQL version
async fn get_postgres_version(pool: &Pool<Postgres>) -> Result<String> {
    let row: (String,) = sqlx::query_as("SELECT version()")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to get version: {}", e))?;
    
    Ok(row.0)
}