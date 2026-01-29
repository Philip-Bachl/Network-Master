use sqlx::{Pool, Sqlite};

use crate::error::Error;

//mod gebaeude;
//mod raum;

pub struct Masterbase {
    pub connection_pool: Pool<Sqlite>,
}

impl Masterbase {
    pub async fn init(connection_string: &str) -> Result<Masterbase, Error> {
        let connection_pool =
            Pool::connect_lazy(connection_string).map_err(Error::DatabaseInitError)?;

        Ok(Masterbase { connection_pool })
    }
}
