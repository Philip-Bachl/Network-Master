use sqlx::{Pool, Sqlite, sqlite::SqliteConnectOptions};

use crate::{
    masterbase_error::MasterbaseError,
    model::{DeviceKind, Dose, DoseZuSwitchPort, Gebaeude, Raum, Schrank, Switch, Switchport},
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

        let mut raeume = Vec::new();
        let mut schraenke = Vec::new();
        for gebaeude in gebaeude_all.iter() {
            for stockwerk_index in 0..STOCKWERK_COUNT {
                for raum_index in 0..RAUM_COUNT {
                    let kommentar =
                        Some(String::from("Testkommentar")).filter(|_| raum_index % 4 == 0);

                    let raum = Raum {
                        ra_id: 0,
                        ra_ge_name: gebaeude.ge_name.clone(),
                        ra_nummer: raum_index.to_string(),
                        ra_stockwerk: stockwerk_index as i32 + 1,
                        ra_kommentar: kommentar,
                    };

                    raeume.push(raum);
                }

                for schrank_index in 0..SCHRANK_COUNT {
                    let kommentar = Some(String::from("Testkommentar"));

                    let schrank = Schrank {
                        sc_id: 0,
                        sc_ge_name: gebaeude.ge_name.clone(),
                        sc_nummer: schrank_index.to_string(),
                        sc_stockwerk: stockwerk_index as i32,
                        sc_kommentar: kommentar,
                    };

                    schraenke.push(schrank);
                }
            }
        }

        let device_kinds = [
            DeviceKind {
                dk_id: 1,
                dk_name: String::from("PC"),
                dk_kommentar: None,
            },
            DeviceKind {
                dk_id: 2,
                dk_name: String::from("Telefon"),
                dk_kommentar: None,
            },
            DeviceKind {
                dk_id: 3,
                dk_name: String::from("PC & Telefon"),
                dk_kommentar: Some(String::from("Speziell durchgeschleußt")),
            },
            DeviceKind {
                dk_id: 4,
                dk_name: String::from("Drucker"),
                dk_kommentar: None,
            },
        ];

        let device_kinds_length = device_kinds.len();
        let mut dosen = Vec::new();
        for (raum_index, _) in raeume.iter().enumerate() {
            for dosen_index in 0..DOSE_COUNT {
                let kommentar =
                    Some(String::from("Testkommentar")).filter(|_| dosen_index % 7 == 0);
                let dose = Dose {
                    do_id: 0,
                    do_ra_id: raum_index as i32 + 1,
                    do_nummer: dosen_index.to_string(),
                    do_dk_id: Some(dosen_index as i32 % device_kinds_length as i32 + 1)
                        .filter(|_| dosen_index % 4 == 0),
                    do_kommentar: kommentar,
                };

                dosen.push(dose);
            }
        }

        let mut switches = Vec::new();
        for (schrank_index, schrank) in schraenke.iter().enumerate() {
            for switch_index in 0..SWITCH_COUNT {
                let kommentar =
                    Some(String::from("Testkommentar")).filter(|_| switch_index % 2 == 0);
                let switch = Switch {
                    sw_name: format!(
                        "{}_{}_{}_{}",
                        schrank.sc_ge_name, schrank.sc_stockwerk, schrank.sc_nummer, switch_index
                    ),
                    sw_sc_id: schrank_index as i32 + 1,
                    sw_ip: format!(
                        "{0}.{0}.{0}.{0}",
                        schrank_index * SWITCH_COUNT + switch_index
                    ),
                    sw_kommentar: kommentar,
                };
                switches.push(switch);
            }
        }

        let mut switchports = Vec::new();
        for switch in switches.iter() {
            for switchport_index in 0..SWITCHPORT_COUNT {
                let vlan = match switchport_index % 10 {
                    0..=2 => 300,
                    3 => 200,
                    _ => 300,
                };
                let kommentar =
                    Some(String::from("Testkommentar")).filter(|_| switchport_index % 2 == 0);

                let switchport = Switchport {
                    sp_id: 0,
                    sp_sw_name: switch.sw_name.clone(),
                    sp_port: format!("fa0/{:02}", switchport_index),
                    sp_vlan: vlan,
                    sp_dot1x: switchport_index % 6 == 0,
                    sp_kommentar: kommentar,
                };

                switchports.push(switchport);
            }
        }

        let dose_zu_switchport_count = dosen.len().min(switchports.len());
        let mut dose_zu_switchports = Vec::new();
        for dose_zu_switchport_index in 0..dose_zu_switchport_count {
            let kommentar =
                Some(String::from("Testkommentar")).filter(|_| dose_zu_switchport_index % 6 == 0);

            let dose_zu_switchport = DoseZuSwitchPort {
                dsz_id: dose_zu_switchport_index as i32 + 1,
                dsz_do_id: dose_zu_switchport_index as i32 + 1,
                dsz_sp_id: dose_zu_switchport_index as i32 + 1,
                dsz_kommentar: kommentar,
            };

            dose_zu_switchports.push(dose_zu_switchport);
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
                        VALUES (NULL, $1, $2, $3, $4)
                    ",
            )
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
                        VALUES (NULL, $1, $2, $3, $4)
                    ",
            )
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
                        VALUES (NULL, $1, $2)
                    ",
            )
            .bind(device_kind.dk_name)
            .bind(device_kind.dk_kommentar)
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
                        VALUES (NULL, $1, $2, $3, $4)
                    ",
            )
            .bind(dose.do_ra_id)
            .bind(&dose.do_nummer)
            .bind(dose.do_dk_id)
            .bind(&dose.do_kommentar)
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
                        VALUES (NULL, $1, $2, $3, $4, $5)
                    ",
            )
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

        println!("Inserting DoseZuSwitchports...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for dose_zu_switchport in dose_zu_switchports {
            sqlx::query(
                "
                        INSERT INTO dzs_dose_zu_switchport
                        VALUES (NULL, $1, $2, $3)
                    ",
            )
            .bind(dose_zu_switchport.dsz_do_id)
            .bind(dose_zu_switchport.dsz_sp_id)
            .bind(&dose_zu_switchport.dsz_kommentar)
            .execute(&mut *transaction)
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();
    }
}
