use diesel::{Connection, ConnectionError, RunQueryDsl, SqliteConnection};

use crate::model::Gebaeude;

pub struct Masterbase {
    connection: SqliteConnection,
}

impl Masterbase {
    pub fn connect(connection_string: &str) -> Result<Masterbase, ConnectionError> {
        SqliteConnection::establish(connection_string).map(|connection| Masterbase { connection })
    }

    pub fn read_gebaeude_all(&mut self) -> Vec<Gebaeude> {
        use crate::schema::ge_gebaeude::dsl::*;
        ge_gebaeude.load(&mut self.connection).unwrap_or_default()
    }
}
