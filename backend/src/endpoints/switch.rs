use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Switch};

#[get("/switch")]
pub async fn read_switch_all(masterbase: &State<Masterbase>) -> Result<Json<Vec<Switch>>, String> {
    sqlx::query_as(
        "
            SELECT * FROM sw_switch
        ",
    )
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|err| err.to_string())
}
#[get("/switch/schrank/<sc_id>")]
pub async fn read_switch_of_schrank(
    masterbase: &State<Masterbase>,
    sc_id: &str,
) -> Result<Json<Vec<Switch>>, String> {
    sqlx::query_as(
        "
            SELECT *
            FROM sw_switch
            WHERE sw_sc_id = $1
        ",
    )
    .bind(sc_id)
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|err| err.to_string())
}
#[get("/switch/gebaeude/<ge_name>")]
pub async fn read_switch_of_gebaeude(
    masterbase: &State<Masterbase>,
    ge_name: &str,
) -> Result<Json<Vec<Switch>>, String> {
    sqlx::query_as(
        "
            SELECT *
            FROM sw_switch as sw
            INNER JOIN sc_schrank as sc ON sw.sw_sc_id = sc.sc_id
            WHERE sc.sc_ge_name = $1
        ",
    )
    .bind(ge_name)
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|err| err.to_string())
}

#[post("/switch", data = "<switch>")]
pub async fn create_switch(
    masterbase: &State<Masterbase>,
    switch: Json<Switch>,
) -> Result<Status, String> {
    sqlx::query(
        "
            INSERT INTO sw_switch
            VALUES ($1, $2, $3, $4)
        ",
    )
    .bind(&switch.sw_name)
    .bind(switch.sw_sc_id)
    .bind(&switch.sw_ip)
    .bind(&switch.sw_kommentar)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Created)
    .map_err(|err| err.to_string())
}
#[post("/switch/port/<prefix>/<count>", data = "<switch>")]
pub async fn create_switch_with_ports(
    masterbase: &State<Masterbase>,
    prefix: &str,
    count: i32,
    switch: Json<Switch>,
) -> Result<Status, String> {
    let mut transaction = masterbase
        .connection_pool
        .begin()
        .await
        .map_err(|err| err.to_string())?;

    sqlx::query(
        "
            INSERT INTO sw_switch
            VALUES ($1, $2, $3, $4)
        ",
    )
    .bind(&switch.sw_name)
    .bind(switch.sw_sc_id)
    .bind(&switch.sw_ip)
    .bind(&switch.sw_kommentar)
    .execute(&mut *transaction)
    .await
    .map_err(|err| err.to_string())?;

    for i in 1..=count {
        sqlx::query(
            "
            INSERT INTO sp_switchport
            VALUES (NULL, $1, $2, $3, $4, $5)
        ",
        )
        .bind(&switch.sw_name)
        .bind(format!("{prefix}{:02}", i))
        .bind(0)
        .bind(false)
        .bind(None::<String>)
        .execute(&mut *transaction)
        .await
        .map_err(|err| err.to_string())?;
    }

    transaction
        .commit()
        .await
        .map(|_| Status::Created)
        .map_err(|err| err.to_string())
}

#[derive(Deserialize)]
pub struct UpdateSwitch {
    sw_name: String,
    switch: Switch,
}

#[put("/switch", data = "<update_switch>")]
pub async fn update_switch(
    masterbase: &State<Masterbase>,
    update_switch: Json<UpdateSwitch>,
) -> Result<Status, String> {
    sqlx::query(
        "
            UPDATE sw_switch
            SET
            sw_name = $1,
            sw_sc_id = $2,
            sw_ip = $3,
            sw_kommentar = $4
            WHERE sw_name = $5
        ",
    )
    .bind(&update_switch.switch.sw_name)
    .bind(update_switch.switch.sw_sc_id)
    .bind(&update_switch.switch.sw_ip)
    .bind(&update_switch.switch.sw_kommentar)
    .bind(&update_switch.sw_name)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Accepted)
    .map_err(|err| err.to_string())
}

#[derive(Deserialize)]
pub struct DeleteSwitch {
    sw_name: String,
}

#[delete("/switch", data = "<delete_switch>")]
pub async fn delete_switch(
    masterbase: &State<Masterbase>,
    delete_switch: Json<DeleteSwitch>,
) -> Result<Status, String> {
    sqlx::query(
        "
            DELETE FROM sp_switchport
            WHERE sp_sw_name = $1
        ",
    )
    .bind(&delete_switch.sw_name)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Ok)
    .map_err(|err| err.to_string())
    .and(
        sqlx::query(
            "
            DELETE FROM sw_switch
            WHERE sw_name = $1
        ",
        )
        .bind(&delete_switch.sw_name)
        .execute(&masterbase.connection_pool)
        .await
        .map(|_| Status::Ok)
        .map_err(|err| err.to_string()),
    )
}
