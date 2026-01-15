use std::env;

use dotenvy::dotenv;

use crate::model::Masterbase;

pub mod model;
pub mod schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;

    let connection_string = env::var("DATABASE_URL")?;
    let database = Masterbase::connect(&connection_string);

    Ok(())
}
