use diesel::{Connection, ConnectionError, SqliteConnection};

pub struct Masterbase {
    connection: SqliteConnection,
}

impl Masterbase {
    pub fn connect(connection_string: &str) -> Result<Masterbase, ConnectionError> {
        SqliteConnection::establish(connection_string).map(|connection| Masterbase { connection })
    }
}
