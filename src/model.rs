use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

//TODO: add ids

#[derive(Serialize, Deserialize, FromRow)]
pub struct Gebaeude {
    pub ge_name: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Raum {
    pub ra_id: i32,
    pub ra_ge_name: String,
    pub ra_nummer: String,
    pub ra_stockwerk: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Schrank {
    pub sc_id: i32,
    pub sc_ge_name: String,
    pub sc_nummer: String,
    pub sc_stockwerk: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Dose {
    pub do_id: i32,
    pub do_ra_id: i32,
    pub do_nummer: String,
    pub do_hat_telefon: bool,
    pub do_hat_pc: bool,
    pub do_hat_drucker: bool,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Switch {
    pub sw_name: String,
    pub sw_sc_id: i32,
    pub sw_ip: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct SwitchZuDose {
    pub szd_sw_name: String,
    pub szd_do_id: i32,
    pub szd_port: String,
    pub szd_vlan: i32,
    pub szd_kommentar: Option<String>,
}
