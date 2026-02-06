use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Dose};

#[get("/dose")]
pub async fn read_dose_all(masterbase: &State<Masterbase>) -> Result<Json<Vec<Dose>>, String> {
    sqlx::query_as(
        "
            SELECT * FROM do_dose
        ",
    )
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|err| err.to_string())
}

#[post("/dose", data = "<dose>")]
pub async fn create_dose(
    masterbase: &State<Masterbase>,
    dose: Json<Dose>,
) -> Result<Status, String> {
    sqlx::query(
        "
            INSERT INTO do_dose VALUES (NULL, $1, $2, $3, $4)
        ",
    )
    .bind(dose.do_id)
    .bind(dose.do_ra_id)
    .bind(&dose.do_nummer)
    .bind(dose.do_dk_id)
    .bind(&dose.do_kommentar)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Created)
    .map_err(|err| err.to_string())
}

#[put("/dose", data = "<dose>")]
pub async fn update_dose(
    masterbase: &State<Masterbase>,
    dose: Json<Dose>,
) -> Result<Status, String> {
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
    .map(|_| Status::Accepted)
    .map_err(|err| err.to_string())
}

#[derive(Deserialize)]
pub struct DeleteDose {
    do_id: i32,
}

#[delete("/dose", data = "<delete_dose>")]
pub async fn delete_dose(
    masterbase: &State<Masterbase>,
    delete_dose: Json<DeleteDose>,
) -> Result<Status, String> {
    sqlx::query(
        "
            DELETE FROM do_dose WHERE do_id = $1
        ",
    )
    .bind(delete_dose.do_id)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Ok)
    .map_err(|err| err.to_string())
}
