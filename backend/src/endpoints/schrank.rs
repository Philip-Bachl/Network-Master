use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Schrank};

#[get("/schrank")]
pub async fn read_schrank_all(
    masterbase: &State<Masterbase>,
) -> Result<Json<Vec<Schrank>>, String> {
    sqlx::query_as(
        "
            SELECT * FROM sc_schrank
        ",
    )
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|err| err.to_string())
}
#[get("/schrank/gebaeude/<ge_name>")]
pub async fn read_schrank_of_gebaeude(
    masterbase: &State<Masterbase>,
    ge_name: &str,
) -> Result<Json<Vec<Schrank>>, String> {
    sqlx::query_as(
        "
            SELECT *
            FROM sc_schrank
            WHERE sc_ge_name = $1
        ",
    )
    .bind(ge_name)
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|err| err.to_string())
}

#[post("/schrank", data = "<schrank>")]
pub async fn create_schrank(
    masterbase: &State<Masterbase>,
    schrank: Json<Schrank>,
) -> Result<Status, String> {
    sqlx::query(
        "
            INSERT INTO sc_schrank
            VALUES (NULL, $1, $2, $3, $4)
        ",
    )
    .bind(&schrank.sc_ge_name)
    .bind(&schrank.sc_nummer)
    .bind(schrank.sc_stockwerk)
    .bind(&schrank.sc_kommentar)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Created)
    .map_err(|err| err.to_string())
}

#[put("/schrank", data = "<schrank>")]
pub async fn update_schrank(
    masterbase: &State<Masterbase>,
    schrank: Json<Schrank>,
) -> Result<Status, String> {
    sqlx::query(
        "
            UPDATE sc_schrank
            SET
            sc_ge_name = $1,
            sc_nummer = $2,
            sc_stockwerk = $3,
            sc_kommentar = $4
            WHERE sc_id = $5
        ",
    )
    .bind(&schrank.sc_ge_name)
    .bind(&schrank.sc_nummer)
    .bind(schrank.sc_stockwerk)
    .bind(&schrank.sc_kommentar)
    .bind(schrank.sc_id)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Accepted)
    .map_err(|err| err.to_string())
}

#[derive(Deserialize)]
pub struct DeleteSchrank {
    sc_id: i32,
}

#[delete("/schrank", data = "<delete_schrank>")]
pub async fn delete_schrank(
    masterbase: &State<Masterbase>,
    delete_schrank: Json<DeleteSchrank>,
) -> Result<Status, String> {
    sqlx::query(
        "
            DELETE FROM sc_schrank WHERE sc_id = $1
        ",
    )
    .bind(delete_schrank.sc_id)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Ok)
    .map_err(|err| err.to_string())
}
