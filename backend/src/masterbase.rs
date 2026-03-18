use sqlx::{Pool, Sqlite, sqlite::SqliteConnectOptions};

use crate::{
    masterbase_error::MasterbaseError,
    model::{DeviceKind, Dose, Gebaeude, Raum, Schrank, Switch, Switchport},
};

//mod gebaeude;
//mod raum;

pub struct Masterbase {
    pub connection_pool: Pool<Sqlite>,
}

impl Masterbase {
    pub async fn init(connection_string: &str) -> Result<Masterbase, MasterbaseError> {
        let connection_pool = Pool::connect_with(
            SqliteConnectOptions::new()
                .filename(connection_string)
                .create_if_missing(true),
        )
        .await
        .map_err(MasterbaseError::DatabaseInit)?;

        Ok(Masterbase { connection_pool })
    }

    pub async fn seed(&self) {
        const GEBAEUDE_COUNT: usize = 3;
        const STOCKWERK_COUNT: usize = 3;
        const RAUM_COUNT: usize = 10;
        const SCHRANK_COUNT: usize = 3;
        const DOSE_COUNT: usize = 20;
        const SWITCH_COUNT: usize = 2;
        const SWITCHPORT_COUNT: usize = 25;

        let mut gebaeude_all = Vec::new();
        for gebaeude_index in 0..GEBAEUDE_COUNT {
            let kommentar = Some(String::from("Testkommentar")).filter(|_| gebaeude_index % 2 == 0);
            let gebaeude = Gebaeude {
                ge_name: format!("Testgebäude {gebaeude_index}"),
                ge_kommentar: kommentar,
            };

            gebaeude_all.push(gebaeude);
        }

        let mut raeume = Vec::with_capacity(gebaeude_all.len() * STOCKWERK_COUNT * RAUM_COUNT);
        for (gebaeude_index, gebaeude) in gebaeude_all.iter().enumerate() {
            for stockwerk_index in 0..STOCKWERK_COUNT {
                for raum_index in 0..RAUM_COUNT {
                    let kommentar =
                        Some(String::from("Testkommentar")).filter(|_| raum_index % 4 == 0);

                    let raum = Raum {
                        ra_id: (raum_index
                            + stockwerk_index * RAUM_COUNT
                            + gebaeude_index * STOCKWERK_COUNT * RAUM_COUNT)
                            as i32,
                        ra_ge_name: gebaeude.ge_name.clone(),
                        ra_nummer: format!("{}{:02}", stockwerk_index, raum_index),
                        ra_stockwerk: (stockwerk_index as i32 - (STOCKWERK_COUNT >> 1) as i32),
                        ra_kommentar: kommentar,
                    };

                    raeume.push(raum);
                }
            }
        }

        let mut schraenke = Vec::new();
        for (gebaeude_index, gebaeude) in gebaeude_all.iter().enumerate() {
            for stockwerk_index in 0..STOCKWERK_COUNT {
                for schrank_index in 0..SCHRANK_COUNT {
                    let kommentar =
                        Some(String::from("Testkommentar")).filter(|_| schrank_index % 2 == 0);

                    let schrank = Schrank {
                        sc_id: (schrank_index
                            + stockwerk_index * SCHRANK_COUNT
                            + gebaeude_index * STOCKWERK_COUNT * SCHRANK_COUNT)
                            as i32,
                        sc_ge_name: gebaeude.ge_name.clone(),
                        sc_nummer: generate_schrank_name(
                            schrank_index as i32,
                            &gebaeude.ge_name,
                            stockwerk_index as i32,
                        ),
                        sc_stockwerk: stockwerk_index as i32,
                        sc_kommentar: kommentar,
                    };

                    schraenke.push(schrank);
                }
            }
        }

        let device_kinds = [
            DeviceKind {
                dk_id: 0,
                dk_name: String::from("pc"),
                dk_kommentar: None,
            },
            DeviceKind {
                dk_id: 1,
                dk_name: String::from("telefon"),
                dk_kommentar: None,
            },
            DeviceKind {
                dk_id: 2,
                dk_name: String::from("pc_plus_telefon"),
                dk_kommentar: Some(String::from("Speziell durchgeschleußt")),
            },
            DeviceKind {
                dk_id: 3,
                dk_name: String::from("drucker"),
                dk_kommentar: None,
            },
        ];

        let mut switches = Vec::new();
        for schrank in schraenke.iter() {
            for switch_index in 0..SWITCH_COUNT {
                let kommentar =
                    Some(String::from("Testkommentar")).filter(|_| switch_index % 3 != 0);

                let switch = Switch {
                    sw_name: generate_switch_name(
                        schrank.sc_id,
                        &schrank.sc_ge_name,
                        schrank.sc_stockwerk,
                        switch_index as i32,
                    ),
                    sw_sc_id: schrank.sc_id,
                    sw_ip: format!("172.196.{}.{}", schrank.sc_id, switch_index),
                    sw_kommentar: kommentar,
                };
                switches.push(switch);
            }
        }

        let mut switchports = Vec::new();
        for (switch_index, switch) in switches.iter().enumerate() {
            for switchport_index in 0..SWITCHPORT_COUNT {
                let vlan = match switchport_index % 10 {
                    0..=2 => 300,
                    3 => 200,
                    _ => 300,
                };
                let kommentar =
                    Some(String::from("Testkommentar")).filter(|_| switchport_index % 2 == 0);

                let switchport = Switchport {
                    sp_id: (switchport_index + switch_index * SWITCHPORT_COUNT) as i32,
                    sp_sw_name: switch.sw_name.clone(),
                    sp_port: format!("fa0/{:02}", switchport_index + 1),
                    sp_vlan: vlan,
                    sp_dot1x: switchport_index % 7 == 0 || switchport_index % 8 == 0,
                    sp_kommentar: kommentar,
                };

                switchports.push(switchport);
            }
        }

        let device_kinds_length = device_kinds.len();
        let mut dosen = Vec::new();
        for raum in raeume.iter() {
            for dosen_index in 0..DOSE_COUNT {
                let kommentar =
                    Some(String::from("Testkommentar")).filter(|_| dosen_index % 7 == 0);

                let dose = Dose {
                    do_id: dosen_index as i32 + raum.ra_id * DOSE_COUNT as i32,
                    do_ra_id: raum.ra_id,
                    do_nummer: dosen_index.to_string(),
                    do_sp_id: None,
                    do_dk_id: Some((dosen_index % device_kinds_length) as i32)
                        .filter(|_| dosen_index % 7 != 0),
                    do_kommentar: kommentar,
                };

                dosen.push(dose);
            }
        }

        for switchport in switchports.iter().step_by(3) {
            dosen[switchport.sp_id as usize].do_sp_id = Some(switchport.sp_id);
        }

        println!("Inserting Gebäude...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for gebaeude in gebaeude_all {
            sqlx::query(
                "
                        INSERT INTO ge_gebaeude VALUES ($1, $2)
                    ",
            )
            .bind(&gebaeude.ge_name)
            .bind(&gebaeude.ge_kommentar)
            .execute(&mut *transaction)
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        println!("Inserting Räume...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for raum in raeume {
            sqlx::query(
                "
                        INSERT INTO ra_raum
                        VALUES ($1, $2, $3, $4, $5)
                    ",
            )
            .bind(raum.ra_id)
            .bind(&raum.ra_ge_name)
            .bind(&raum.ra_nummer)
            .bind(raum.ra_stockwerk)
            .bind(&raum.ra_kommentar)
            .execute(&mut *transaction)
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        println!("Inserting Schränke...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for schrank in schraenke {
            sqlx::query(
                "
                        INSERT INTO sc_schrank
                        VALUES ($1, $2, $3, $4, $5)
                    ",
            )
            .bind(schrank.sc_id)
            .bind(&schrank.sc_ge_name)
            .bind(&schrank.sc_nummer)
            .bind(schrank.sc_stockwerk)
            .bind(&schrank.sc_kommentar)
            .execute(&mut *transaction)
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        println!("Inserting DeviceKinds...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for device_kind in device_kinds {
            sqlx::query(
                "
                        INSERT INTO dk_device_kind
                        VALUES ($1, $2, $3)
                    ",
            )
            .bind(device_kind.dk_id)
            .bind(device_kind.dk_name)
            .bind(device_kind.dk_kommentar)
            .execute(&mut *transaction)
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        println!("Inserting Switches...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for switch in switches {
            sqlx::query(
                "
                        INSERT INTO sw_switch
                        VALUES ($1, $2, $3, $4)
                    ",
            )
            .bind(&switch.sw_name)
            .bind(switch.sw_sc_id)
            .bind(&switch.sw_ip)
            .bind(&switch.sw_kommentar)
            .execute(&mut *transaction)
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        println!("Inserting Switchports...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for switchport in switchports {
            sqlx::query(
                "
                        INSERT INTO sp_switchport
                        VALUES ($1, $2, $3, $4, $5, $6)
                    ",
            )
            .bind(switchport.sp_id)
            .bind(&switchport.sp_sw_name)
            .bind(&switchport.sp_port)
            .bind(switchport.sp_vlan)
            .bind(switchport.sp_dot1x)
            .bind(&switchport.sp_kommentar)
            .execute(&mut *transaction)
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        println!("Inserting Dosen...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for dose in dosen {
            sqlx::query(
                "
                        INSERT INTO do_dose
                        VALUES ($1, $2, $3, $4, $5, $6)
                    ",
            )
            .bind(dose.do_id)
            .bind(dose.do_ra_id)
            .bind(&dose.do_nummer)
            .bind(dose.do_sp_id)
            .bind(dose.do_dk_id)
            .bind(&dose.do_kommentar)
            .execute(&mut *transaction)
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();
    }
}

fn pretty_stockwerk_number(stockwerk: i32) -> String {
    match stockwerk {
        0 => String::from("EG"),
        s @ ..=-1 => format!("{}UG", s.abs()),
        s @ 1.. => format!("{}OG", s),
    }
}

fn generate_schrank_name(sc_id: i32, sc_ge_name: &str, sc_stockwerk: i32) -> String {
    let mut chars = sc_ge_name.chars();

    format!(
        "{}{}-{}-{}.{}",
        chars.next().unwrap(),
        chars.last().unwrap(),
        pretty_stockwerk_number(sc_stockwerk),
        sc_stockwerk,
        sc_id
    )
}

fn generate_switch_name(sc_id: i32, sc_ge_name: &str, sc_stockwerk: i32, suffix: i32) -> String {
    let mut chars = sc_ge_name.chars();

    format!(
        "{}{}-{}-{}.{}/{}",
        chars.next().unwrap(),
        chars.last().unwrap(),
        pretty_stockwerk_number(sc_stockwerk),
        sc_stockwerk,
        sc_id,
        suffix
    )
}
