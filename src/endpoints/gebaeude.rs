use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};

use crate::{
    MasterState,
    masterbase::{CreateEntityRequest, DeleteEntityRequest, UpdateEntityRequest},
    model::Gebaeude,
};

//TODO: add read endpoints with filters
#[get("/gebaeude")]
pub async fn get_gebaeude_all(state: &State<MasterState>) -> Result<Json<Vec<Gebaeude>>, Status> {
    let Ok(mut database) = state.database.lock() else {
        println!("ERROR: Poisioned Lock detected at 'get_gebaeude_all'");
        return Err(Status::InternalServerError);
    };

    let gebaeude_all = match database.read_gebaeude_all() {
        Ok(ge_al) => ge_al,
        Err(err) => {
            println!("ERROR: at 'get_gebaeude_all': {err}");
            return Err(Status::InternalServerError);
        }
    };

    Ok(Json(gebaeude_all))
}

#[post("/gebaeude", data = "<gebaeude>")]
pub async fn add_gebaede(
    state: &State<MasterState>,
    gebaeude: Json<CreateEntityRequest<Gebaeude>>,
) -> Status {
    let Ok(mut database) = state.database.lock() else {
        println!("ERROR: Poisioned Lock detected at 'add_gebaeude'");
        return Status::InternalServerError;
    };

    let gebaeude = gebaeude.0;

    match database.create_gebaeude(gebaeude) {
        Ok(_) => Status::Created,
        Err(err) => {
            println!("ERROR: at 'add_gebaeude': {err}");
            Status::InternalServerError
        }
    }
}

#[put("/gebaeude", data = "<update_gebaeude>")]
pub async fn change_gebaede(
    state: &State<MasterState>,
    update_gebaeude: Json<UpdateEntityRequest<String, Gebaeude>>,
) -> Status {
    let Ok(mut database) = state.database.lock() else {
        println!("ERROR: Poisioned Lock detected at 'change_gebaeude'");
        return Status::InternalServerError;
    };

    let update_gebaeude = update_gebaeude.0;

    match database.update_gebaede(update_gebaeude) {
        Ok(_) => Status::Created,
        Err(err) => {
            println!("ERROR: at 'change_gebaeude': {err}");
            Status::InternalServerError
        }
    }
}

#[delete("/gebaeude", data = "<delete_gebaeude>")]
pub async fn remove_gebaeude(
    state: &State<MasterState>,
    delete_gebaeude: Json<DeleteEntityRequest<String>>,
) -> Status {
    let Ok(mut database) = state.database.lock() else {
        println!("ERROR: Poisioned Lock detected at 'remove_gebaeude'");
        return Status::InternalServerError;
    };

    let delete_gebaeude = delete_gebaeude.0;

    match database.delete_gebaude(delete_gebaeude) {
        Ok(_) => Status::Ok,
        Err(err) => {
            println!("ERROR: at 'remove_gebaeude': {err}");
            Status::InternalServerError
        }
    }
}
