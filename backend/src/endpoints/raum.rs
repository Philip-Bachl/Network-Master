use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Raum};

#[get("/raum")]
pub async fn read_raum_all(masterbase: &State<Masterbase>) -> Result<Json<Vec<Raum>>, String> {
    sqlx::query_as(
        "
            SELECT * FROM ra_raum
        ",
    )
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|err| err.to_string())
}

#[post("/raum", data = "<raum>")]
pub async fn create_raum(
    masterbase: &State<Masterbase>,
    raum: Json<Raum>,
) -> Result<Status, String> {
    sqlx::query(
        "
            INSERT INTO ra_raum
            VALUES (NULL, $1, $2, $3, $4)
        ",
    )
    .bind(&raum.ra_ge_name)
    .bind(&raum.ra_nummer)
    .bind(raum.ra_stockwerk)
    .bind(&raum.ra_kommentar)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Created)
    .map_err(|err| err.to_string())
}

#[put("/raum", data = "<raum>")]
pub async fn update_raum(
    masterbase: &State<Masterbase>,
    raum: Json<Raum>,
) -> Result<Status, String> {
    sqlx::query(
        "
            UPDATE ra_raum
            SET
            ra_ge_name = $1,
            ra_nummer = $2,
            ra_stockwerk = $3,
            ra_kommentar = $4
            WHERE ra_id = $5
        ",
    )
    .bind(&raum.ra_ge_name)
    .bind(&raum.ra_nummer)
    .bind(raum.ra_stockwerk)
    .bind(&raum.ra_kommentar)
    .bind(raum.ra_id)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Accepted)
    .map_err(|err| err.to_string())
}

#[derive(Deserialize)]
pub struct DeleteRaum {
    ra_id: i32,
}

#[delete("/raum", data = "<delete_raum>")]
pub async fn delete_raum(
    masterbase: &State<Masterbase>,
    delete_raum: Json<DeleteRaum>,
) -> Result<Status, String> {
    sqlx::query(
        "
            DELETE FROM ra_raum
            WHERE ra_id = $1
        ",
    )
    .bind(delete_raum.ra_id)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Ok)
    .map_err(|err| err.to_string())
}
