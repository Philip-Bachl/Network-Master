use diesel::{Connection, ConnectionError, Selectable, SqliteConnection, prelude::Queryable};

use crate::schema;

pub struct Masterbase {
    connection: SqliteConnection,
}

impl Masterbase {
    pub fn connect(connection_string: &str) -> Result<Masterbase, ConnectionError> {
        SqliteConnection::establish(connection_string).map(|connection| Masterbase { connection })
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::do_dose)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Dose {
    pub do_nummer: String,
    pub do_ra_nummer: i32,
    pub do_ra_stockwerk: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::ge_gebaeude)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Gebaeude {
    pub ge_name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::ra_raum)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Raum {
    pub ra_ge_name: String,
    pub ra_stockwerk: i32,
    pub ra_nummer: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::sc_schrank)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Schrank {
    pub sc_nummer: i32,
    pub sc_stockwerk: i32,
    pub sc_ge_name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::sd_switch_zu_dose)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SwitchZuDose {
    pub sd_sw_ip: String,
    pub sd_do_nummer: String,
    pub sd_do_ra_nummer: i32,
    pub sd_do_ra_stockwerk: i32,
    pub sd_switchport: String,
    pub sd_hat_telefon: bool,
    pub sd_hat_pc: bool,
    pub sd_hat_drucker: bool,
    pub sd_kommentar: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::sw_switch)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Switch {
    pub sw_ip: String,
    pub sw_sc_nummer: i32,
    pub sw_sc_stockwerk: i32,
}
