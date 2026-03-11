use rocket::{State, get, serde::json::Json};
use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::masterbase::Masterbase;

#[derive(Serialize, FromRow)]
pub struct SwitchportDetails {
    sp_port: String,
    sp_dot1x: bool,
    do_id: Option<i32>,
    do_nummer: Option<String>,
    dk_name: Option<String>,
}

#[get("/switch/<sw_name>")]
pub async fn read_switch_details(
    masterbase: &State<Masterbase>,
    sw_name: &str,
) -> Result<Json<Vec<SwitchportDetails>>, String> {
    sqlx::query_as(
        "
            SELECT sp.sp_port, sp.sp_dot1x, do.do_id, do.do_nummer, dk.dk_name
            FROM sp_switchport as sp
            LEFT JOIN do_dose as do ON do.do_sp_id = sp.sp_id
            LEFT JOIN dk_device_kind as dk ON do.do_dk_id = dk.dk_id
            WHERE sp.sp_sw_name = $1
            ORDER BY sp.sp_port
        ",
    )
    .bind(sw_name)
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|err| err.to_string())
}
