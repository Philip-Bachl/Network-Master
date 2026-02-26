use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};

//TODO: add ids

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ImplicitClone)]
pub struct Gebaeude {
    pub ge_name: String,
    pub ge_kommentar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Raum {
    pub ra_id: i32,
    pub ra_ge_name: String,
    pub ra_nummer: String,
    pub ra_stockwerk: i32,
    pub ra_kommentar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ImplicitClone)]
pub struct Schrank {
    pub sc_id: i32,
    pub sc_ge_name: String,
    pub sc_nummer: String,
    pub sc_stockwerk: i32,
    pub sc_kommentar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceKind {
    pub dk_id: i32,
    pub dk_name: String,
    pub dk_kommentar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dose {
    pub do_id: i32,
    pub do_ra_id: i32,
    pub do_nummer: String,
    pub do_dk_id: Option<i32>,
    pub do_kommentar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ImplicitClone)]
pub struct Switch {
    pub sw_name: String,
    pub sw_sc_id: i32,
    pub sw_ip: String,
    pub sw_kommentar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Switchport {
    pub sp_id: i32,
    pub sp_sw_name: String,
    pub sp_port: String,
    pub sp_vlan: i32,
    pub sp_dot1x: bool,
    pub sp_kommentar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DoseZuSwitchport {
    pub dsz_id: i32,
    pub dsz_do_id: i32,
    pub dsz_sp_id: i32,
    pub dsz_kommentar: Option<String>,
}
