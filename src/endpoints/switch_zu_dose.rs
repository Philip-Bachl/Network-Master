use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::SwitchZuDose};

#[get("/switch-zu-dose")]
pub async fn read_switch_zu_dose(
    masterbase: &State<Masterbase>,
) -> Result<Json<Vec<SwitchZuDose>>, Status> {
    sqlx::query_as("SELECT * FROM szd_switch_zu_dose")
        .fetch_all(&masterbase.connection_pool)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[post("/switch-zu-dose", data = "<switch_zu_dose>")]
pub async fn create_switch_zu_dose(
    masterbase: &State<Masterbase>,
    switch_zu_dose: Json<SwitchZuDose>,
) -> Status {
    sqlx::query("INSERT INTO szd_switch_zu_dose VALUES ($1, $2, $3, $4, $5)")
        .bind(&switch_zu_dose.szd_sw_name)
        .bind(switch_zu_dose.szd_do_id)
        .bind(&switch_zu_dose.szd_port)
        .bind(switch_zu_dose.szd_vlan)
        .execute(&masterbase.connection_pool)
        .await
        .map_or_else(|_| Status::InternalServerError, |_| Status::Created)
}

#[derive(Deserialize)]
pub struct UpdateSwitchZuDose {
    szd_sw_name: String,
    szd_do_id: i32,
    switch_zu_dose: SwitchZuDose,
}

#[put("/switch-zu-dose", data = "<update_switch_zu_dose>")]
pub async fn update_switch_zu_dose(
    masterbase: &State<Masterbase>,
    update_switch_zu_dose: Json<UpdateSwitchZuDose>,
) -> Status {
    sqlx::query(
        "
            UPDATE szd_switch_zu_dose
            SET szd_sw_name = $1, szd_do_id = $2, szd_port = $3, szd_vlan = $4, szd_Kommentar = $5
            WHERE szd_sw_name = $6 AND szd_do_id = $7
        ",
    )
    .bind(&update_switch_zu_dose.switch_zu_dose.szd_sw_name)
    .bind(update_switch_zu_dose.switch_zu_dose.szd_do_id)
    .bind(&update_switch_zu_dose.switch_zu_dose.szd_port)
    .bind(update_switch_zu_dose.switch_zu_dose.szd_vlan)
    .bind(&update_switch_zu_dose.switch_zu_dose.szd_kommentar)
    //
    .bind(&update_switch_zu_dose.szd_sw_name)
    .bind(update_switch_zu_dose.szd_do_id)
    //
    .execute(&masterbase.connection_pool)
    .await
    .map_or_else(|_| Status::InternalServerError, |_| Status::Accepted)
}

#[derive(Deserialize)]
pub struct DeleteSwitchZuDose {
    szd_sw_name: String,
    szd_do_id: i32,
}

#[delete("/switch_zu_dose", data = "<delete_switch_zu_dose>")]
pub async fn delete_switch_zu_dose(
    masterbase: &State<Masterbase>,
    delete_switch_zu_dose: Json<DeleteSwitchZuDose>,
) -> Status {
    sqlx::query("DELETE FROM szd_switch_zu_dose WHERE szd_sw_name = $1 AND szd_do_id = $2")
        .bind(&delete_switch_zu_dose.szd_sw_name)
        .bind(delete_switch_zu_dose.szd_do_id)
        .execute(&masterbase.connection_pool)
        .await
        .map_or_else(|_| Status::InternalServerError, |_| Status::Ok)
}
