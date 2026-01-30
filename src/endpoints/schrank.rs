use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Schrank};

#[get("/schrank")]
pub async fn read_schrank_all(
    masterbase: &State<Masterbase>,
) -> Result<Json<Vec<Schrank>>, Status> {
    let all_schraenke = sqlx::query_as("SELECT * FROM sc_schrank")
        .fetch_all(&masterbase.connection_pool)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(all_schraenke))
}

#[post("/schrank", data = "<schrank>")]
pub async fn create_schrank(masterbase: &State<Masterbase>, schrank: Json<Schrank>) -> Status {
    match sqlx::query("INSERT INTO sc_schrank VALUES ($1, $2, $3)")
        .bind(&schrank.sc_nummer)
        .bind(schrank.sc_stockwerk)
        .bind(&schrank.sc_ge_name)
        .execute(&masterbase.connection_pool)
        .await
    {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(Deserialize)]
pub struct UpdateSchrank {
    sc_nummer: String,
    sc_stockwerk: i32,
    sc_ge_name: String,
    schrank: Schrank,
}

#[put("/schrank", data = "<update_schrank>")]
pub async fn update_schrank(
    masterbase: &State<Masterbase>,
    update_schrank: Json<UpdateSchrank>,
) -> Status {
    match sqlx::query("UPDATE sc_schrank SET sc_nummer = $1, sc_stockwerk = $2, sc_ge_name = $3 WHERE sc_nummer = $1, sc_stockwerk = $2, sc_ge_name = $3")
        .bind(&update_schrank.schrank.sc_nummer)
        .bind(update_schrank.schrank.sc_stockwerk)
        .bind(&update_schrank.schrank.sc_ge_name)
        //
        .bind(&update_schrank.sc_nummer)
        .bind(update_schrank.sc_stockwerk)
        .bind(&update_schrank.sc_ge_name)
        //
        .execute(&masterbase.connection_pool)
        .await
    {
        Ok(_) => Status::Accepted,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(Deserialize)]
pub struct DeleteSchrank {
    sc_nummer: String,
    sc_stockwerk: i32,
    sc_ge_name: String,
}

#[delete("/schrank", data = "<delete_schrank>")]
pub async fn delete_schrank(
    masterbase: &State<Masterbase>,
    delete_schrank: Json<DeleteSchrank>,
) -> Status {
    match sqlx::query(
        "DELETE FROM sc_schrank WHERE sc_nummer = $1, sc_stockwerk = $2, sc_ge_name = $3",
    )
    .bind(&delete_schrank.sc_nummer)
    .bind(delete_schrank.sc_stockwerk)
    .bind(&delete_schrank.sc_ge_name)
    .execute(&masterbase.connection_pool)
    .await
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
