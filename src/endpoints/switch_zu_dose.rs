use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::SwitchZuDose};

#[get("/switch-zu-dose")]
pub async fn read_switch_zu_dose(
    masterbase: &State<Masterbase>,
) -> Result<Json<Vec<SwitchZuDose>>, Status> {
    let switch_zu_dosen = sqlx::query_as("SELECT * FROM sd_switch_zu_dose")
        .fetch_all(&masterbase.connection_pool)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(switch_zu_dosen))
}

#[post("/switch-zu-dose", data = "<switch_zu_dose>")]
pub async fn create_switch_zu_dose(
    masterbase: &State<Masterbase>,
    switch_zu_dose: Json<SwitchZuDose>,
) -> Status {
    match sqlx::query("INSERT INTO sd_switch_zu_dose VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(&switch_zu_dose.sd_do_nummer)
        .bind(&switch_zu_dose.sd_do_ra_nummer)
        .bind(switch_zu_dose.sd_do_ra_stockwerk)
        .bind(&switch_zu_dose.sd_do_ra_ge_name)
        .bind(&switch_zu_dose.sd_sw_ip)
        .bind(&switch_zu_dose.sd_switchport)
        .bind(&switch_zu_dose.sd_kommentar)
        .execute(&masterbase.connection_pool)
        .await
    {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(Deserialize)]
pub struct UpdateSwitchZuDose {
    sd_do_nummer: String,
    sd_do_ra_nummer: String,
    sd_do_ra_stockwerk: i32,
    sd_do_ra_ge_name: String,
    switch_zu_dose: SwitchZuDose,
}

#[put("/switch-zu-dose", data = "<update_switch_zu_dose>")]
pub async fn update_switch_zu_dose(
    masterbase: &State<Masterbase>,
    update_switch_zu_dose: Json<UpdateSwitchZuDose>,
) -> Status {
    match sqlx::query(
        "
            UPDATE sd_switch_zu_dose
            SET
            sd_do_nummer = $1,
            sd_do_ra_nummer = $2,
            sd_ra_stockwerk = $3,
            sd_do_ra_ge_name = $4,
            sd_sw_ip = $5,
            sd_switchport = $6,
            sd_kommentar = $7
            WHERE
            sd_do_nummer = $8,
            sd_do_ra_nummer = $9,
            sd_ra_stockwerk = $10,
            sd_do_ra_ge_name = $11
        ",
    )
    .bind(&update_switch_zu_dose.switch_zu_dose.sd_do_nummer)
    .bind(&update_switch_zu_dose.switch_zu_dose.sd_do_ra_nummer)
    .bind(update_switch_zu_dose.switch_zu_dose.sd_do_ra_stockwerk)
    .bind(&update_switch_zu_dose.switch_zu_dose.sd_do_ra_ge_name)
    .bind(&update_switch_zu_dose.switch_zu_dose.sd_sw_ip)
    .bind(&update_switch_zu_dose.switch_zu_dose.sd_switchport)
    .bind(&update_switch_zu_dose.switch_zu_dose.sd_kommentar)
    //
    .bind(&update_switch_zu_dose.sd_do_nummer)
    .bind(&update_switch_zu_dose.sd_do_ra_nummer)
    .bind(update_switch_zu_dose.sd_do_ra_stockwerk)
    .bind(&update_switch_zu_dose.sd_do_ra_ge_name)
    //
    .execute(&masterbase.connection_pool)
    .await
    {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(Deserialize)]
pub struct DeleteSwitchZuDose {
    sd_do_nummer: String,
    sd_do_ra_nummer: String,
    sd_do_ra_stockwerk: i32,
    sd_do_ra_ge_name: String,
}

#[delete("/switch_zu_dose", data = "<delete_switch_zu_dose>")]
pub async fn delete_switch_zu_dose(
    masterbase: &State<Masterbase>,
    delete_switch_zu_dose: Json<DeleteSwitchZuDose>,
) -> Status {
    match sqlx::query(
        "
            DELETE FROM sd_switch_zu_dose
            WHERE
            sd_do_nummer = $1,
            sd_do_ra_nummer = $2,
            sd_ra_stockwerk = $3,
            sd_do_ra_ge_name = $4
        ",
    )
    .bind(&delete_switch_zu_dose.sd_do_nummer)
    .bind(&delete_switch_zu_dose.sd_do_ra_nummer)
    .bind(delete_switch_zu_dose.sd_do_ra_stockwerk)
    .bind(&delete_switch_zu_dose.sd_do_ra_ge_name)
    .execute(&masterbase.connection_pool)
    .await
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

/*
    a suprise tool that will help us later:

    SELECT do.*
    FROM sd_switch_zu_dose as sd INNER JOIN do_dose as do
    ON
    do.do_nummer = sd.sd_do_nummer AND
    do.do_ra_nummer = sd.sd_do_ra_nummer AND
    do.do_ra_stockwerk = sd.sd_do_ra_stockwerk AND
    do.do_ra_ge_name = sd.sd_do_ra_ge_name;
*/
