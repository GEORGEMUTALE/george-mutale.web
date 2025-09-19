use std::error::Error;
use sqlx::Row;
struct Book {
    pub id: String,
    pub title: String
}

async fn create(hello: &Book, pool:&sqlx::PgPool) -> Result<(), Box<dyn Error>>{
    let query = "INSERT INTO hello(id,title) VALUES (001,GEORGE'S JOURNEY)";
    sqlx::query(query)
    .bind(&hello.id)
    .bind(&hello.title)
    .execute(pool)
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgresql://neondb_owner:npg_o6KUEqWCj1yS@ep-misty-river-adh7iopy-pooler.c-2.us-east-1.aws.neon.tech/neondb?sslmode=require&channel_binding=require";
    let pool= sqlx::postgres::PgPool::connect(url).await?;
    
    sqlx::migrate!("./migrations").run(&pool).await?;
    let hello = Book{
        id: "002".to_string(),
        title:"Martin".to_string()
    };

    create(&hello, &pool).await?;
    Ok(())
}