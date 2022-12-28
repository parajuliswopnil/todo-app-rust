use clap::Error;
use serde::{Deserialize, Serialize};
use diesel::{prelude::*, connection};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use dotenv::dotenv;
use std::env;
use std::fmt::format;
use diesel::connection::SimpleConnection;

use crate::error_handler::CustomError;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// embed_migrations!();

// lazy_static! {
//     static ref POOL: Pool = {
//         let db_url = env::var("DATABASE_URL").expect("Database Url not set");
//         let manager = ConnectionManager::<PgConnection>::new(db_url);
//         Pool::new(manager).expect("Failed to create db pool")
//     };
// }

// pub fn init() {
//     lazy_static::initialize(&POOL);
//     let conn = connection().expect("Failed to connect to db");
//     embedded_migrations::run(&conn).unwrap();
// }

// pub fn connection() -> Result<DBConnection, CustomError> {
//     POOL.get().map_err(|e| CustomError { error_status_code: (500), error_message: format!("Failed getting db connection {}", e) })
// }

pub fn db_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}