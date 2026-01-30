use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Gebaeude};

//TODO: add read endpoints with filters
#[get("/gebaeude")]
pub async fn read_gebaeude_all(
    masterbase: &State<Masterbase>,
) -> Result<Json<Vec<Gebaeude>>, Status> {
    let all_gebaeude = sqlx::query_as("SELECT * FROM ge_gebaeude")
        .fetch_all(&masterbase.connection_pool)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(all_gebaeude))
}

#[post("/gebaeude", data = "<gebaeude>")]
pub async fn create_gebaede(masterbase: &State<Masterbase>, gebaeude: Json<Gebaeude>) -> Status {
    match sqlx::query("INSERT INTO ge_gebaeude VALUES ($1)")
        .bind(&gebaeude.ge_name)
        .execute(&masterbase.connection_pool)
        .await
    {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(Deserialize)]
pub struct UpdateGebaeude {
    ge_name: String,
    gebaeude: Gebaeude,
}

#[put("/gebaeude", data = "<update_gebaeude>")]
pub async fn update_gebaeude(
    masterbase: &State<Masterbase>,
    update_gebaeude: Json<UpdateGebaeude>,
) -> Status {
    match sqlx::query("UPDATE ge_gebaeude SET ge_name = $1 WHERE ge_name = $2")
        .bind(&update_gebaeude.gebaeude.ge_name)
        .bind(&update_gebaeude.ge_name)
        .execute(&masterbase.connection_pool)
        .await
    {
        Ok(_) => Status::Accepted,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(Deserialize)]
pub struct DeleteGebaeude {
    ge_name: String,
}

#[delete("/gebaeude", data = "<delete_gebaeude>")]
pub async fn delete_gebaeude(
    masterbase: &State<Masterbase>,
    delete_gebaeude: Json<DeleteGebaeude>,
) -> Status {
    match sqlx::query("DELETE FROM ge_gebaeude WHERE ge_name = $1")
        .bind(&delete_gebaeude.ge_name)
        .execute(&masterbase.connection_pool)
        .await
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
