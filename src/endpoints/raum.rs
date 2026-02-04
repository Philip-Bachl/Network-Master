use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Raum};

#[get("/raum")]
pub async fn read_raeume_all(masterbase: &State<Masterbase>) -> Result<Json<Vec<Raum>>, Status> {
    sqlx::query_as("SELECT * FROM ra_raum")
        .fetch_all(&masterbase.connection_pool)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[post("/raum", data = "<raum>")]
pub async fn create_raum(masterbase: &State<Masterbase>, raum: Json<Raum>) -> Status {
    sqlx::query("INSERT INTO ra_raum VALUES (NULL, $1, $2, $3)")
        .bind(&raum.ra_ge_name)
        .bind(&raum.ra_nummer)
        .bind(raum.ra_stockwerk)
        .execute(&masterbase.connection_pool)
        .await
        .map_or(Status::InternalServerError, |_| Status::Created)
}

#[derive(Deserialize)]
pub struct UpdateRaum {
    ra_id: i32,
    raum: Raum,
}

#[put("/raum", data = "<update_raum>")]
pub async fn update_raum(masterbase: &State<Masterbase>, update_raum: Json<UpdateRaum>) -> Status {
    sqlx::query(
        "UPDATE ra_raum SET ra_ge_name = $1, ra_nummer = $2, ra_stockwerk = $3 WHERE ra_id = $4",
    )
    .bind(&update_raum.raum.ra_ge_name)
    .bind(&update_raum.raum.ra_nummer)
    .bind(update_raum.raum.ra_stockwerk)
    .bind(update_raum.ra_id)
    .execute(&masterbase.connection_pool)
    .await
    .map_or_else(|_| Status::InternalServerError, |_| Status::Accepted)
}

#[derive(Deserialize)]
pub struct DeleteRaum {
    ra_id: i32,
}

#[delete("/raum", data = "<delete_raum>")]
pub async fn delete_raum(masterbase: &State<Masterbase>, delete_raum: Json<DeleteRaum>) -> Status {
    sqlx::query("DELETE FROM ra_raum WHERE ra_id = $1")
        .bind(delete_raum.ra_id)
        .execute(&masterbase.connection_pool)
        .await
        .map_or_else(|_| Status::InternalServerError, |_| Status::Ok)
}
