use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Dose};

#[get("/dose")]
pub async fn read_dosen_all(masterbase: &State<Masterbase>) -> Result<Json<Vec<Dose>>, Status> {
    let all_dosen = sqlx::query_as("SELECT * FROM do_dose")
        .fetch_all(&masterbase.connection_pool)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(all_dosen))
}

#[post("/dose", data = "<dose>")]
pub async fn create_dose(masterbase: &State<Masterbase>, dose: Json<Dose>) -> Status {
    match sqlx::query("INSERT INTO do_dose VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(&dose.do_nummer)
        .bind(&dose.do_ra_nummer)
        .bind(dose.do_ra_stockwerk)
        .bind(&dose.do_ra_ge_name)
        .bind(dose.do_hat_telefon)
        .bind(dose.do_hat_pc)
        .bind(dose.do_hat_drucker)
        .execute(&masterbase.connection_pool)
        .await
    {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(Deserialize)]
pub struct UpdateDose {
    do_nummer: String,
    do_ra_nummer: String,
    do_ra_stockwerk: i32,
    do_ra_ge_name: String,

    dose: Dose,
}

#[put("/dose", data = "<update_dose>")]
pub async fn update_dose(masterbase: &State<Masterbase>, update_dose: Json<UpdateDose>) -> Status {
    match sqlx::query(
        "
            UPDATE do_dose
            SET do_nummer = $1, do_ra_nummer = $2, do_ra_stockwerk = $3, do_rage_name = $4, do_hat_telefon = $5, do_hat_pc = $6, do_hat_drucker = $7
            WHERE do_nummer = $5, do_ra_nummer = $6, do_ra_stockwerk = $7, do_rage_name = $8
        ",
    )
    .bind(&update_dose.dose.do_nummer)
    .bind(&update_dose.dose.do_ra_nummer)
    .bind(update_dose.dose.do_ra_stockwerk)
    .bind(&update_dose.dose.do_ra_ge_name)
    .bind(update_dose.dose.do_hat_telefon)
    .bind(update_dose.dose.do_hat_pc)
    .bind(update_dose.dose.do_hat_drucker)
    //
    .bind(&update_dose.do_nummer)
    .bind(&update_dose.do_ra_nummer)
    .bind(update_dose.do_ra_stockwerk)
    .bind(&update_dose.do_ra_ge_name)
    //
    .execute(&masterbase.connection_pool)
    .await
    {
        Ok(_) => Status::Accepted,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(Deserialize)]
pub struct DeleteDose {
    do_nummer: String,
    do_ra_nummer: String,
    do_ra_stockwerk: i32,
    do_ra_ge_name: String,
}

#[delete("/dose", data = "<delete_dose>")]
pub async fn delete_dose(masterbase: &State<Masterbase>, delete_dose: Json<DeleteDose>) -> Status {
    match sqlx::query(
        "DELETE FROM do_dose WHERE do_nummer = $1, do_ra_nummer = $2, do_ra_stockwerk = $3, do_rage_name = $4",
    )
    .bind(&delete_dose.do_nummer)
    .bind(&delete_dose.do_ra_nummer)
    .bind(delete_dose.do_ra_stockwerk)
    .bind(&delete_dose.do_ra_ge_name)
    .execute(&masterbase.connection_pool)
    .await
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
