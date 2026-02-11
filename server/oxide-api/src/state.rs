use std::env;
use sqlx::PgPool;
use oxide_data::PostgresContext;

pub struct AppState {
    postgres_context: PostgresContext,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let connection_string =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or system env");
        let pg_pool = PgPool::connect(connection_string.as_str()).await?;
        Ok(Self{
            postgres_context: PostgresContext::new(pg_pool.clone()),
        })

    }
}