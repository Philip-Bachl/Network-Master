use diesel::{
    Connection, ConnectionError, ExpressionMethods, RunQueryDsl, SqliteConnection, insert_into,
};

use crate::{model::Gebaeude, schema::ge_gebaeude};

pub struct Masterbase {
    connection: SqliteConnection,
}

impl Masterbase {
    pub fn connect(connection_string: &str) -> Result<Masterbase, ConnectionError> {
        SqliteConnection::establish(connection_string).map(|connection| Masterbase { connection })
    }

    pub fn read_gebaeude_all(&mut self) -> Result<Vec<Gebaeude>, diesel::result::Error> {
        use crate::schema::ge_gebaeude::dsl::*;

        ge_gebaeude.load(&mut self.connection)
    }

    pub fn create_gebaeude(&mut self, gebaeude: Gebaeude) -> Result<usize, diesel::result::Error> {
        use crate::schema::ge_gebaeude::dsl::*;

        let new_gebaeude = ge_name.eq(gebaeude.ge_name);

        insert_into(ge_gebaeude)
            .values(new_gebaeude)
            .execute(&mut self.connection)
    }
}
