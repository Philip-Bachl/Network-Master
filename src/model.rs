use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

//TODO: add ids

#[derive(Serialize, Deserialize, FromRow)]
pub struct Gebaeude {
    pub ge_name: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Raum {
    pub ra_nummer: String,
    pub ra_stockwerk: i32,
    pub ra_ge_name: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Schrank {
    pub sc_nummer: String,
    pub sc_stockwerk: i32,
    pub sc_ge_name: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Dose {
    pub do_nummer: String,
    pub do_ra_nummer: String,
    pub do_ra_stockwerk: i32,
    pub do_ra_ge_name: String,
    pub do_hat_telefon: bool,
    pub do_hat_pc: bool,
    pub do_hat_drucker: bool,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Switch {
    pub sw_ip: String,
    pub sw_sc_nummer: Option<String>,
    pub sw_sc_stockwerk: Option<i32>,
    pub sw_sc_ge_name: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct SwitchZuDose {
    pub sd_do_nummer: String,
    pub sd_do_ra_nummer: String,
    pub sd_do_ra_stockwerk: i32,
    pub sd_do_ra_ge_name: String,
    pub sd_sw_ip: String,
    pub sd_switchport: String,
    pub sd_kommentar: Option<String>,
}
