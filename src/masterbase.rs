use sqlx::{Pool, Sqlite, sqlite::SqliteConnectOptions};

use crate::error::Error;

//mod gebaeude;
//mod raum;

pub struct Masterbase {
    pub connection_pool: Pool<Sqlite>,
}

impl Masterbase {
    pub async fn init(connection_string: &str) -> Result<Masterbase, Error> {
        let connection_pool = Pool::connect_lazy_with(
            SqliteConnectOptions::new()
                .filename(connection_string)
                .create_if_missing(true),
        );

        Ok(Masterbase { connection_pool })
    }
}
