use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Switch};

#[get("/switch")]
pub async fn read_switches_all(
    masterbase: &State<Masterbase>,
) -> Result<Json<Vec<Switch>>, Status> {
    let switches_all = sqlx::query_as("SELECT * FROM sw_switch")
        .fetch_all(&masterbase.connection_pool)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(switches_all))
}

#[post("/switch", data = "<switch>")]
pub async fn create_switch(masterbase: &State<Masterbase>, switch: Json<Switch>) -> Status {
    match sqlx::query("INSERT INTO sw_switch VALUES ($1, $2, $3, $4)")
        .bind(&switch.sw_ip)
        .bind(&switch.sw_sc_nummer)
        .bind(switch.sw_sc_stockwerk)
        .bind(&switch.sw_sc_ge_name)
        .execute(&masterbase.connection_pool)
        .await
    {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(Deserialize)]
pub struct UpdateSwitch {
    sw_ip: String,
    switch: Switch,
}

#[put("/switch", data = "<update_switch>")]
pub async fn update_switch(
    masterbase: &State<Masterbase>,
    update_switch: Json<UpdateSwitch>,
) -> Status {
    match sqlx::query(
        "
            UPDATE sw_switch
            SET sw_ip = $1, sw_sc_nummer = $2, sw_sc_stockwerk = $3, sw_sc_ge_name = $4
            WHERE sw_ip = $5
        ",
    )
    .bind(&update_switch.switch.sw_ip)
    .bind(&update_switch.switch.sw_sc_nummer)
    .bind(update_switch.switch.sw_sc_stockwerk)
    .bind(&update_switch.switch.sw_sc_ge_name)
    //
    .bind(&update_switch.sw_ip)
    //
    .execute(&masterbase.connection_pool)
    .await
    {
        Ok(_) => Status::Accepted,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(Deserialize)]
pub struct DeleteSwitch {
    sw_ip: String,
}

#[delete("/switch", data = "<delete_switch>")]
pub async fn delete_switch(
    masterbase: &State<Masterbase>,
    delete_switch: Json<DeleteSwitch>,
) -> Status {
    match sqlx::query("DELETE FROM sw_switch WHERE sw_ip = $1")
        .bind(&delete_switch.sw_ip)
        .execute(&masterbase.connection_pool)
        .await
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
