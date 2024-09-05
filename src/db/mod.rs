use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use dotenv::dotenv;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn run_migrations(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).expect("Error running migrations");
}
