use serde::{Deserialize, Serialize};
use yew::AttrValue;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Gebaeude {
    pub ge_name: String,
    pub ge_kommentar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Raum {
    pub ra_id: i32,
    pub ra_ge_name: String,
    pub ra_nummer: String,
    pub ra_stockwerk: i32,
    pub ra_kommentar: Option<String>,
}
impl Raum {
    pub fn pretty_raum_number(&self) -> AttrValue {
        //TODO: will likely change to just be ra_nummer as ra_nummer will include the ra_stockwerk (makes input easier)
        let tail = if self.ra_nummer.len() == 1 {
            &format!("0{}", self.ra_nummer)
        } else {
            &self.ra_nummer
        };

        format!("{}{}", self.ra_stockwerk, tail).into()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Schrank {
    pub sc_id: i32,
    pub sc_ge_name: String,
    pub sc_nummer: String,
    pub sc_stockwerk: i32,
    pub sc_kommentar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceKind {
    pub dk_id: i32,
    pub dk_name: String,
    pub dk_kommentar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Dose {
    pub do_id: i32,
    pub do_ra_id: i32,
    pub do_nummer: String,
    pub do_sp_id: Option<i32>,
    pub do_dk_id: Option<i32>,
    pub do_kommentar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Switch {
    pub sw_name: String,
    pub sw_sc_id: i32,
    pub sw_ip: String,
    pub sw_kommentar: Option<String>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Switchport {
    pub sp_id: i32,
    pub sp_sw_name: String,
    pub sp_port: String,
    pub sp_vlan: i32,
    pub sp_dot1x: bool,
    pub sp_kommentar: Option<String>,
}
