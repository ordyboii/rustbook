use std::str::FromStr;

use fake::Fake;
use sqlx::FromRow;
use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};

use fake::faker::name::en::Name;
use fake::faker::internet::en::SafeEmail;

#[derive(Debug, FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
    created_at: String,
}

async fn seed_users(pool: &SqlitePool, count: usize) -> Result<(), sqlx::Error> {
    for _ in 0..count {
        let name: String = Name().fake();
        let email: String = SafeEmail().fake();
        sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
            .bind(name)
            .bind(email)
            .execute(pool)
            .await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let options = SqliteConnectOptions::from_str("sqlite://users.db")?
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    seed_users(&pool, 10).await?; 

    let users: Vec<User> = sqlx::query_as::<_, User>(
        "SELECT id, name, email, strftime('%Y-%m-%dT%H:%M:%S', created_at) as created_at FROM users"
    )
    .fetch_all(&pool)
    .await?;

    for user in users {
        println!("{:?}", user);
    }

    Ok(())
}
