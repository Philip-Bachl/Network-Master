use rocket::{State, get, serde::json::Json};
use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::masterbase::Masterbase;

#[derive(Serialize, FromRow)]
pub struct SwitchportDetail {
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
) -> Result<Json<Vec<SwitchportDetail>>, String> {
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

#[derive(Serialize, FromRow)]
pub struct DoseDetails {
    do_id: i32,
    do_nummer: String,
    dk_name: Option<String>,
    sp_port: Option<String>,
    sp_dot1x: Option<bool>,
    sp_vlan: Option<i32>,
    sw_name: Option<String>,
    sw_ip: Option<String>,
}

#[get("/raum/<ra_id>")]
pub async fn read_raum_details(
    masterbase: &State<Masterbase>,
    ra_id: i32,
) -> Result<Json<Vec<DoseDetails>>, String> {
    sqlx::query_as(
        "
            SELECT do.do_id, do.do_nummer, dk.dk_name, sp.sp_port, sp.sp_dot1x, sp.sp_vlan, sw.sw_name, sw.sw_ip 
            FROM ra_raum as ra
            INNER JOIN do_dose as do ON do.do_ra_id = ra.ra_id
            LEFT JOIN dk_device_kind as dk ON do.do_dk_id = dk.dk_id
            LEFT JOIN sp_switchport as sp ON do.do_sp_id = sp.sp_id
            LEFT JOIN sw_switch as sw ON sp.sp_sw_name = sw.sw_name
            WHERE ra.ra_id = $1
            ORDER BY do.do_nummer
        ",
    )
    .bind(ra_id)
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|err| err.to_string())
}
