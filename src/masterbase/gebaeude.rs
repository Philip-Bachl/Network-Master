use diesel::{
    ExpressionMethods, RunQueryDsl, delete, insert_into, query_dsl::methods::FilterDsl, update,
};

use crate::{
    masterbase::{CreateEntityRequest, DeleteEntityRequest, Masterbase, UpdateEntityRequest},
    model::Gebaeude,
};

impl Masterbase {
    pub fn create_gebaeude(
        &mut self,
        gebaeude: CreateEntityRequest<Gebaeude>,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::ge_gebaeude::dsl::*;

        let new_gebaeude = ge_name.eq(gebaeude.entity.ge_name);

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
        update_gebaeude: UpdateEntityRequest<String, Gebaeude>,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::ge_gebaeude::dsl::*;

        update(ge_gebaeude.filter(ge_name.eq(update_gebaeude.key)))
            .set(ge_name.eq(update_gebaeude.entity.ge_name))
            .execute(&mut self.connection)
    }

    pub fn delete_gebaude(
        &mut self,
        delete_gebaeude: DeleteEntityRequest<String>,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::ge_gebaeude::dsl::*;

        delete(ge_gebaeude.filter(ge_name.eq(delete_gebaeude.key))).execute(&mut self.connection)
    }
}
