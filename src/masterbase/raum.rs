use diesel::{ExpressionMethods, RunQueryDsl, insert_into};

use crate::{
    masterbase::{CreateEntityRequest, Masterbase},
    model::Raum,
    schema::ra_raum,
};

impl Masterbase {
    pub fn create_raum(
        &mut self,
        create_raum: CreateEntityRequest<Raum>,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::ra_raum::dsl::*;

        let raum = create_raum.entity;

        insert_into(ra_raum)
            .values((
                ra_ge_name.eq(raum.ra_ge_name),
                ra_nummer.eq(raum.ra_nummer),
                ra_stockwerk.eq(raum.ra_stockwerk),
            ))
            .execute(&mut self.connection)
    }

    pub fn read_raum_all(&mut self) -> Result<Vec<Raum>, diesel::result::Error> {
        use crate::schema::ra_raum::dsl::*;

        ra_raum.load(&mut self.connection)
    }
}
