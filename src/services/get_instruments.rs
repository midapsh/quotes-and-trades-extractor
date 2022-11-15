use sqlx::postgres::PgPoolOptions;

async fn get_instruments(exchange: &str) -> Result<Vec<String>, Error> {
    let conn_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());

    let pg_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&conn_str)
        .await?;
    
    // TODO(hspadim): Add log here
    let mut tx = pg_pool.begin().await?;
    let instruments: Vec<String> = sqlx::query!(
            "select instrument from instruments where exchange = $1;",
            exchange,
        )
        .execute(&mut tx)
        .await?
        .rows_affected();

    Ok(instruments)
}