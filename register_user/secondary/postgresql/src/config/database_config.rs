use std::sync::Arc;
use diesel::{r2d2, PgConnection};
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use lazy_static::lazy_static;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Arc<Pool> = {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(database_url);

        let pool = r2d2::Pool::builder()
        .max_size(15) // Or any number, depending on your requirements.
            .build(manager)
            .expect("Failed to create pool.");

        Arc::new(pool)
    };
}

pub fn get_connection() -> r2d2::PooledConnection<ConnectionManager<PgConnection>> {
    POOL.get().expect("Failed to get connection from pool.")
}
