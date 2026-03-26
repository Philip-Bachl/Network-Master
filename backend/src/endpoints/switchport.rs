use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use serde::Deserialize;

use crate::{masterbase::Masterbase, model::Switchport};

#[get("/switchport")]
pub async fn read_switchport_all(
    masterbase: &State<Masterbase>,
) -> Result<Json<Vec<Switchport>>, String> {
    sqlx::query_as(
        "
            SELECT * FROM sp_switchport
        ",
    )
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|err| err.to_string())
}
#[get("/switchport/switch/<sw_id>")]
pub async fn read_switchport_of_switch(
    masterbase: &State<Masterbase>,
    sw_id: &str,
) -> Result<Json<Vec<Switchport>>, String> {
    sqlx::query_as(
        "
            SELECT *
            FROM sp_switchport
            WHERE sp_sw_id = $1
        ",
    )
    .bind(sw_id)
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|err| err.to_string())
}

#[post("/switchport", data = "<switchport>")]
pub async fn create_switchport(
    masterbase: &State<Masterbase>,
    switchport: Json<Switchport>,
) -> Result<Status, String> {
    sqlx::query(
        "
            INSERT INTO sp_switchport
            VALUES (NULL, $1, $2, $3, $4, $5)
        ",
    )
    .bind(switchport.sp_sw_id)
    .bind(&switchport.sp_port)
    .bind(switchport.sp_vlan)
    .bind(switchport.sp_dot1x)
    .bind(&switchport.sp_kommentar)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Created)
    .map_err(|err| err.to_string())
}

#[put("/switchport", data = "<switchport>")]
pub async fn update_switchport(
    masterbase: &State<Masterbase>,
    switchport: Json<Switchport>,
) -> Result<Status, String> {
    sqlx::query(
        "
            UPDATE sp_switchport
            SET
            sp_sw_id = $1,
            sp_port = $2,
            sp_vlan = $3,
            sp_dot1x = $4,
            sp_kommentar = $5
            WHERE
            sp_id = $6
        ",
    )
    .bind(switchport.sp_sw_id)
    .bind(&switchport.sp_port)
    .bind(switchport.sp_vlan)
    .bind(switchport.sp_dot1x)
    .bind(&switchport.sp_kommentar)
    .bind(switchport.sp_id)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Accepted)
    .map_err(|err| err.to_string())
}

#[derive(Deserialize)]
pub struct DeleteSwitchport {
    sp_id: i32,
}

#[delete("/switchport", data = "<delete_switchport>")]
pub async fn delete_switchport(
    masterbase: &State<Masterbase>,
    delete_switchport: Json<DeleteSwitchport>,
) -> Result<Status, String> {
    sqlx::query(
        "
            DELETE FROM sp_switchport
            WHERE sp_id = $1
        ",
    )
    .bind(delete_switchport.sp_id)
    .execute(&masterbase.connection_pool)
    .await
    .map(|_| Status::Ok)
    .map_err(|err| err.to_string())
}
