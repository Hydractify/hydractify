use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

/// Establishes connection with the database.
pub fn establish_connection(url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let connection_manager = ConnectionManager::<PgConnection>::new(url);

    Pool::builder()
        .test_on_check_out(true)
        .build(connection_manager)
        .expect("Failed to create database connection pool.")
}
