use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Raum};

#[get("/raum")]
pub async fn read_raeume_all(masterbase: &State<Masterbase>) -> Result<Json<Vec<Raum>>, Status> {
    let all_raeume = sqlx::query_as("SELECT * FROM ra_raum")
        .fetch_all(&masterbase.connection_pool)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(all_raeume))
}

#[post("/raum", data = "<raum>")]
pub async fn create_raum(masterbase: &State<Masterbase>, raum: Json<Raum>) -> Status {
    match sqlx::query(
        "INSERT INTO ra_raum (ra_nummer, ra_stockwerk, ra_ge_name) VALUES ($1, $2, $3)",
    )
    .bind(&raum.ra_nummer)
    .bind(raum.ra_stockwerk)
    .bind(&raum.ra_ge_name)
    .execute(&masterbase.connection_pool)
    .await
    {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(Deserialize)]
pub struct UpdateRaum {
    ra_nummer: String,
    ra_stockwerk: i32,
    ra_ge_name: String,
    raum: Raum,
}

#[put("/raum", data = "<update_raum>")]
pub async fn update_raum(masterbase: &State<Masterbase>, update_raum: Json<UpdateRaum>) -> Status {
    match sqlx::query(
        "UPDATE ra_raum SET ra_nummer = $1, ra_stockwerk = $2, ra_ge_name = $3 WHERE ra_ge_name = $4, ra_stockwerk = $5, ra_nummer = $6",
    )
    .bind(&update_raum.raum.ra_nummer)
    .bind(update_raum.raum.ra_stockwerk)
    .bind(&update_raum.raum.ra_ge_name)    
    //
    .bind(&update_raum.ra_nummer)
    .bind(update_raum.ra_stockwerk)
    .bind(&update_raum.ra_ge_name)
    //
    .execute(&masterbase.connection_pool)
    .await
    {
        Ok(_) => Status::Accepted,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(Deserialize)]
pub struct DeleteRaum {
    ra_nummer: String,
    ra_stockwerk: i32,
    ra_ge_name: String,
}

#[delete("/raum", data = "<delete_raum>")]
pub async fn delete_raum(masterbase: &State<Masterbase>, delete_raum: Json<DeleteRaum>) -> Status {
    match sqlx::query("DELETE FROM ra_raum WHERE ra_nummer = $1, ra_stockwerk = $2, ra_ge_name = $3")
    .bind(&delete_raum.ra_nummer)
    .bind(delete_raum.ra_stockwerk)
    .bind(&delete_raum.ra_ge_name)
    .execute(&masterbase.connection_pool).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
