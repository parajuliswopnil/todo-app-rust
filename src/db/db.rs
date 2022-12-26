use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn db_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}