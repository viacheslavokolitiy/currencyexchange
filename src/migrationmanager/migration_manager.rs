use sqlx::PgPool;

struct MigrationManager {
    pool: PgPool,
}

impl MigrationManager {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    ///
    /// Migrates schemas
    fn migrate(&self) -> anyhow::Result<()> {
        let rnt = tokio::runtime::Runtime::new().expect("Failed to create runtime thread pool");
        rnt.block_on(async {
            sqlx::migrate!("./migrations")
                .run(&self.pool)
                .await
                .expect("Failed to migrate the database");
        });
        Ok(())
    }
}

#[cfg(test)]
mod migrationmanager_spec {
    use crate::utils::env_parser::EnvParser;
    use crate::migrationmanager::migration_manager::MigrationManager;
    use crate::utils::database_connector::DatabaseConnector;

    #[test]
    fn should_migrate() {
        let parser = EnvParser::new();
        let conn_manager = DatabaseConnector::new(
            parser.database_url(),
            parser.max_connections()
        );
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime thread pool");
        let pool = runtime.block_on(async {
            let x = conn_manager.connect().await;
            x
        });
        let manager = MigrationManager::new(pool);
        let result = manager.migrate();
        assert!(result.is_ok());
    }
}