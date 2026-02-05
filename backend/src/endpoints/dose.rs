use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Dose};

#[get("/dose")]
pub async fn read_dose_all(masterbase: &State<Masterbase>) -> Result<Json<Vec<Dose>>, Status> {
    sqlx::query_as(
        "
            SELECT * FROM do_dose
        ",
    )
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[post("/dose", data = "<dose>")]
pub async fn create_dose(masterbase: &State<Masterbase>, dose: Json<Dose>) -> Status {
    sqlx::query(
        "
            INSERT INTO do_dose VALUES (NULL, $1, $2, $3, $4)
        ",
    )
    .bind(dose.do_id)
    .bind(&dose.do_ra_id)
    .bind(&dose.do_nummer)
    .bind(dose.do_dk_id)
    .bind(&dose.do_kommentar)
    .execute(&masterbase.connection_pool)
    .await
    .map_or_else(|_| Status::InternalServerError, |_| Status::Created)
}

#[put("/dose", data = "<dose>")]
pub async fn update_dose(masterbase: &State<Masterbase>, dose: Json<Dose>) -> Status {
    sqlx::query(
        "
            UPDATE do_dose
            SET
            do_ra_id = $1, do_nummer = $2, do_dk_id = $3, do_kommentar = $4
            WHERE do_id = $5
        ",
    )
    .bind(dose.do_ra_id)
    .bind(&dose.do_nummer)
    .bind(dose.do_dk_id)
    .bind(&dose.do_kommentar)
    .bind(dose.do_id)
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
    sqlx::query(
        "
            DELETE FROM do_dose WHERE do_id = $1
        ",
    )
    .bind(delete_dose.do_id)
    .execute(&masterbase.connection_pool)
    .await
    .map_or_else(|_| Status::InternalServerError, |_| Status::Ok)
}
