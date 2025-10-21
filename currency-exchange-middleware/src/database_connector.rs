use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub struct DatabaseConnector {
    database_url: String,
    max_connections: u32,
}

impl DatabaseConnector {
    pub fn new<S: Into<String>, C: Into<u32>>(database_url: S, max_connections: C) -> Self {
        Self {
            database_url: database_url.into(),
            max_connections: max_connections.into(),
        }
    }

    ///
    /// Connects to database and returns PgPool if success
    pub async fn connect(&self) -> PgPool {
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(self.database_url.as_str())
            .await
            .expect("Failed to connect to database")
    }
}