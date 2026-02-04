use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Dose};

#[get("/dose")]
pub async fn read_dosen_all(masterbase: &State<Masterbase>) -> Result<Json<Vec<Dose>>, Status> {
    sqlx::query_as("SELECT * FROM do_dose")
        .fetch_all(&masterbase.connection_pool)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[post("/dose", data = "<dose>")]
pub async fn create_dose(masterbase: &State<Masterbase>, dose: Json<Dose>) -> Status {
    sqlx::query("INSERT INTO do_dose VALUES ($1, $2, $3, $4, $5)")
        .bind(dose.do_ra_id)
        .bind(&dose.do_nummer)
        .bind(dose.do_hat_telefon)
        .bind(dose.do_hat_pc)
        .bind(dose.do_hat_drucker)
        .execute(&masterbase.connection_pool)
        .await
        .map_or_else(|_| Status::InternalServerError, |_| Status::Created)
}

#[derive(Deserialize)]
pub struct UpdateDose {
    do_id: i32,
    dose: Dose,
}

#[put("/dose", data = "<update_dose>")]
pub async fn update_dose(masterbase: &State<Masterbase>, update_dose: Json<UpdateDose>) -> Status {
    sqlx::query(
        "
            UPDATE do_dose
            SET do_ra_id = $1, do_nummer = $2, do_hat_telefon = $3, do_hat_pc = $4, do_hat_drucker = $5
            WHERE do_id = $6
        ",
    )
    .bind(update_dose.dose.do_ra_id)
    .bind(update_dose.dose.do_hat_telefon)
    .bind(update_dose.dose.do_hat_pc)
    .bind(update_dose.dose.do_hat_drucker)
    .bind(update_dose.do_id)
    .execute(&masterbase.connection_pool)
    .await
    .map_or_else(|_| Status::InternalServerError, |_| Status::Accepted)
}

#[derive(Deserialize)]
pub struct DeleteDose {
    do_id: i32,
}

#[delete("/dose", data = "<delete_dose>")]
pub async fn delete_dose(masterbase: &State<Masterbase>, delete_dose: Json<DeleteDose>) -> Status {
    sqlx::query("DELETE FROM do_dose WHERE do_id = $1")
        .bind(delete_dose.do_id)
        .execute(&masterbase.connection_pool)
        .await
        .map_or_else(|_| Status::InternalServerError, |_| Status::Ok)
}
