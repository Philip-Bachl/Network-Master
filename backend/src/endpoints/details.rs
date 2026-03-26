use rocket::{State, get, serde::json::Json};
use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::masterbase::Masterbase;

#[derive(Serialize, FromRow)]
pub struct SwitchportDetail {
    sp_id: i32,
    sp_sw_id: i32,
    sp_port: String,
    sp_vlan: i32,
    sp_dot1x: bool,
    sp_kommentar: Option<String>,

    do_id: Option<i32>,
    do_nummer: Option<String>,
    dk_name: Option<String>,
}

#[get("/switch/<sw_id>")]
pub async fn read_switch_details(
    masterbase: &State<Masterbase>,
    sw_id: &str,
) -> Result<Json<Vec<SwitchportDetail>>, String> {
    sqlx::query_as(
        "
            SELECT sp.*, do.do_id, do.do_nummer, dk.dk_name
            FROM sp_switchport as sp
            LEFT JOIN do_dose as do ON do.do_sp_id = sp.sp_id
            LEFT JOIN dk_device_kind as dk ON do.do_dk_id = dk.dk_id
            WHERE sp.sp_sw_id = $1
            ORDER BY sp.sp_port
        ",
    )
    .bind(sw_id)
    .fetch_all(&masterbase.connection_pool)
    .await
    .map(Json)
    .map_err(|err| err.to_string())
}

#[derive(Serialize, FromRow)]
pub struct DoseDetail {
    do_id: i32,
    do_ra_id: i32,
    do_nummer: String,
    do_sp_id: Option<i32>,
    do_dk_id: Option<i32>,
    do_kommentar: Option<String>,

    dk_name: Option<String>,

    sp_id: Option<i32>,
    sp_sw_id: Option<i32>,
    sp_port: Option<String>,
    sp_vlan: Option<i32>,
    sp_dot1x: Option<bool>,
    sp_kommentar: Option<String>,

    sw_name: Option<String>,
    sw_ip: Option<String>,
} //SMALL TODO: sw_id and sp_sw_id should always have the same value here. fix in frontend

#[get("/raum/<ra_id>")]
pub async fn read_raum_details(
    masterbase: &State<Masterbase>,
    ra_id: i32,
) -> Result<Json<Vec<DoseDetail>>, String> {
    sqlx::query_as(
        "
            SELECT do.*, dk.dk_name, sp.*, sw.sw_name, sw.sw_ip 
            FROM ra_raum as ra
            INNER JOIN do_dose as do ON do.do_ra_id = ra.ra_id
            LEFT JOIN dk_device_kind as dk ON do.do_dk_id = dk.dk_id
            LEFT JOIN sp_switchport as sp ON do.do_sp_id = sp.sp_id
            LEFT JOIN sw_switch as sw ON sp.sp_sw_id = sw.sw_id
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
