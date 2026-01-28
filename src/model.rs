use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

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

pub struct Schrank {
    pub sc_nummer: i32,
    pub sc_stockwerk: i32,
    pub sc_ge_name: String,
}

pub struct Dose {
    pub do_nummer: String,
    pub do_ra_ge_name: String,
    pub do_ra_nummer: i32,
    pub do_ra_stockwerk: i32,
}

pub struct Switch {
    pub sw_ip: String,
    pub sw_sc_nummer: i32,
    pub sw_sc_stockwerk: i32,
    pub sw_sc_ge_name: i32,
}

pub struct SwitchZuDose {
    pub sd_sw_ip: String,
    pub sd_do_nummer: String,
    pub sd_do_ra_nummer: i32,
    pub sd_do_ra_stockwerk: i32,
    pub sd_do_ra_ge_name: String,
    pub sd_switchport: String,
    pub sd_hat_telefon: bool,
    pub sd_hat_pc: bool,
    pub sd_hat_drucker: bool,
    pub sd_kommentar: Option<String>,
}
