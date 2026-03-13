pub mod models;
pub mod schema;

use crate::db::DbError::MigrationError;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use log::{debug, info};
use std::sync::Arc;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

/// Alias type for the connection pool
pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// Alias type for a pooled database connection
pub type DbConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

/// Database state shared across the application
#[derive(Clone)]
pub struct DbState {
    pub pool: Arc<DbPool>,
}

impl DbState {
    pub fn new(pool: DbPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }

    pub fn get_conn(&self) -> Result<DbConnection, DbError> {
        self.pool
            .get()
            .map_err(|e| DbError::PoolError(e.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Pool error: {0}")]
    PoolError(String),

    #[error("Request Error: {0}")]
    QueryError(#[from] diesel::result::Error),

    #[error("Migration error: {0}")]
    MigrationError(String),
}

pub fn create_pool() -> Result<DbPool, DbError> {
    debug!("Creating database connection pool...");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);

    Pool::builder()
        .max_size(10)
        .min_idle(Some(2))
        .build(manager)
        .map_err(|e| DbError::PoolError(e.to_string()))
        .inspect(|_| info!("Database connection pool created successfully."))
}

pub fn establish_connection() -> SqliteConnection {
    debug!("Establishing database connection...");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    match SqliteConnection::establish(&database_url) {
        Ok(conn) => {
            info!("Database connection established successfully.");
            conn
        }
        Err(e) => {
            panic!("Error connecting to {}: {}", database_url, e);
        }
    }
}

pub fn run_migrations(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Running database migrations...");

    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(migrations) => {
            if migrations.is_empty() {
                debug!("No pending migrations to run.");
            } else {
                info!("Migrations applied successfully.");
                for migration in &migrations {
                    info!("Applied migration: {}", migration);
                }
            }
            Ok(())
        }
        Err(e) => Err(Box::new(MigrationError(e.to_string()))),
    }
}
