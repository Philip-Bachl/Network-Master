use diesel::{
    Connection, ConnectionError, ExpressionMethods, RunQueryDsl, SqliteConnection, delete,
    insert_into, query_dsl::methods::FilterDsl, update,
};
use serde::Deserialize;

use crate::model::Gebaeude;

#[derive(Deserialize)]
pub struct UpdateGebaeude {
    ge_name: String,
    new_gebaeude: Gebaeude,
}

#[derive(Deserialize)]
pub struct DeleteGebaeude {
    ge_name: String,
}

pub struct Masterbase {
    connection: SqliteConnection,
}

impl Masterbase {
    pub fn connect(connection_string: &str) -> Result<Masterbase, ConnectionError> {
        SqliteConnection::establish(connection_string).map(|connection| Masterbase { connection })
    }

    pub fn create_gebaeude(&mut self, gebaeude: Gebaeude) -> Result<usize, diesel::result::Error> {
        use crate::schema::ge_gebaeude::dsl::*;

        let new_gebaeude = ge_name.eq(gebaeude.ge_name);

        insert_into(ge_gebaeude)
            .values(new_gebaeude)
            .execute(&mut self.connection)
    }

    pub fn read_gebaeude_all(&mut self) -> Result<Vec<Gebaeude>, diesel::result::Error> {
        use crate::schema::ge_gebaeude::dsl::*;

        ge_gebaeude.load(&mut self.connection)
    }

    pub fn update_gebaede(
        &mut self,
        update_gebaeude: UpdateGebaeude,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::ge_gebaeude::dsl::*;

        update(ge_gebaeude.filter(ge_name.eq(update_gebaeude.ge_name)))
            .set(ge_name.eq(update_gebaeude.new_gebaeude.ge_name))
            .execute(&mut self.connection)
    }

    pub fn delete_gebaude(
        &mut self,
        delete_gebaeude: DeleteGebaeude,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::ge_gebaeude::dsl::*;

        delete(ge_gebaeude.filter(ge_name.eq(delete_gebaeude.ge_name)))
            .execute(&mut self.connection)
    }
}
