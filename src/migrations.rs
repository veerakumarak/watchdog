use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    use diesel::pg::PgConnection;
    use diesel::prelude::*;

    // Migrations must run on a standard synchronous connection
    let mut connection = PgConnection::establish(db_url)?;

    println!("Searching for pending migrations...");

    connection.run_pending_migrations(MIGRATIONS)
        .map_err(|v| format!("Migration error: {}", v))?;

    println!("✅ Migrations completed successfully.");
    Ok(())
}