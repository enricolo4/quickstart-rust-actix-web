use std::sync::Arc;
use std::env;
use diesel::{r2d2, PgConnection};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use dotenvy::dotenv;
use lazy_static::lazy_static;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, HarnessWithOutput, MigrationHarness};
use log::info;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type PollConnectionDataBase = PooledConnection<ConnectionManager<PgConnection>>;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

lazy_static! {
    pub static ref POOL: Arc<Pool> = {
        dotenv().ok();
        let database_url = env::var("REGISTER_USER_DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let manager = ConnectionManager
            ::<PgConnection>
            ::new(database_url);

        let pool = Pool::builder()
            .max_size(15)
            .build(manager)
            .expect("Failed to get connection for migrations.");

        let mut connection  = pool
            .get()
            .expect("Failed to get connection from pool.");

        let applied_migrations = connection
            .applied_migrations()
            .expect("Failed to get applied migrations");

        for migration in &applied_migrations {
            info!("Already applied migration: {}", migration);
        }

        let mut harness = HarnessWithOutput
        ::write_to_stdout(&mut connection);

        harness.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations.");

        let all_applied_migrations = connection
            .applied_migrations()
            .expect("Failed to get applied migrations");

         if let Some(last_migration) = all_applied_migrations.last() {
            info!("Last applied migration: {}", last_migration);
        } else {
            info!("No migrations have been applied yet.");
        }

        // Generate schema after running migrations
        generate_schema();

        Arc::new(pool)
    };
}

fn generate_schema() {
    use std::process::Command;
    use std::env;
    
    info!("Generating schema...");
    
    let output = Command::new("diesel")
        .args(&["print-schema"])
        .current_dir("register_user/secondary/postgresql")
        .env("DATABASE_URL", env::var("REGISTER_USER_DATABASE_URL").unwrap_or_default())
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                let schema_content = String::from_utf8_lossy(&result.stdout);
                if let Err(e) = std::fs::write("register_user/secondary/postgresql/src/user/schema/schema.rs", schema_content.as_bytes()) {
                    info!("Failed to write schema file: {}", e);
                } else {
                    info!("Schema generated successfully!");
                }
            } else {
                info!("Failed to generate schema: {}", String::from_utf8_lossy(&result.stderr));
            }
        }
        Err(e) => {
            info!("Failed to execute diesel command: {}", e);
        }
    }
}

pub fn get_connection() -> PollConnectionDataBase {
    POOL.get().expect("Failed to get connection from pool.")
}
