use diesel::{Connection, ConnectionError, SqliteConnection};
use serde::Deserialize;

mod gebaeude;
mod raum;

#[derive(Deserialize)]
pub struct CreateEntityRequest<E> {
    entity: E,
}

#[derive(Deserialize)]
pub struct UpdateEntityRequest<K, E> {
    key: K,
    entity: E,
}

#[derive(Deserialize)]
pub struct DeleteEntityRequest<K> {
    key: K,
}

pub struct Masterbase {
    connection: SqliteConnection,
}

impl Masterbase {
    pub fn connect(connection_string: &str) -> Result<Masterbase, ConnectionError> {
        SqliteConnection::establish(connection_string).map(|connection| Masterbase { connection })
    }
}
