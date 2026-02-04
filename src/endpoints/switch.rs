use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Switch};

#[get("/switch")]
pub async fn read_switches_all(
    masterbase: &State<Masterbase>,
) -> Result<Json<Vec<Switch>>, Status> {
    sqlx::query_as("SELECT * FROM sw_switch")
        .fetch_all(&masterbase.connection_pool)
        .await
        .map(|all_switches| Json(all_switches))
        .map_err(|_| Status::InternalServerError)
}

#[post("/switch", data = "<switch>")]
pub async fn create_switch(masterbase: &State<Masterbase>, switch: Json<Switch>) -> Status {
    sqlx::query("INSERT INTO sw_switch VALUES ($1, $2, $3)")
        .bind(&switch.sw_name)
        .bind(switch.sw_sc_id)
        .bind(&switch.sw_ip)
        .execute(&masterbase.connection_pool)
        .await
        .map_or_else(|_| Status::InternalServerError, |_| Status::Created)
}

#[derive(Deserialize)]
pub struct UpdateSwitch {
    sw_name: String,
    switch: Switch,
}

#[put("/switch", data = "<update_switch>")]
pub async fn update_switch(
    masterbase: &State<Masterbase>,
    update_switch: Json<UpdateSwitch>,
) -> Status {
    sqlx::query("UPDATE sw_switch SET sw_name = $1, sw_sc_id = $2, sw_ip = $3 WHERE sw_name = $4")
        .bind(&update_switch.switch.sw_name)
        .bind(&update_switch.switch.sw_sc_id)
        .bind(&update_switch.switch.sw_ip)
        .bind(&update_switch.sw_name)
        .execute(&masterbase.connection_pool)
        .await
        .map_or_else(|_| Status::InternalServerError, |_| Status::Accepted)
}

#[derive(Deserialize)]
pub struct DeleteSwitch {
    sw_name: String,
}

#[delete("/switch", data = "<delete_switch>")]
pub async fn delete_switch(
    masterbase: &State<Masterbase>,
    delete_switch: Json<DeleteSwitch>,
) -> Status {
    sqlx::query("DELETE FROM sw_switch WHERE sw_name = $1")
        .bind(&delete_switch.sw_name)
        .execute(&masterbase.connection_pool)
        .await
        .map_or_else(|_| Status::InternalServerError, |_| Status::Ok)
}
