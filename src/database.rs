use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

/// Establishes connection with the database.
pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    // Ensure the .env file is loaded into the environment.
    dotenv::dotenv().ok();

    let connection_manager = ConnectionManager::<PgConnection>::new(
        std::env::var("DATABASE_URL")
            .ok()
            .expect("The DATABASE_URL environment is not present."),
    );

    Pool::builder()
        .test_on_check_out(true)
        .build(connection_manager)
        .expect("Failed to create database connection pool.")
}
