use sqlx::{Pool, Sqlite, sqlite::SqliteConnectOptions};

use crate::{
    masterbase_error::MasterbaseError,
    model::{Dose, Gebaeude, Raum, Schrank, Switch, SwitchZuDose},
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
        let all_gebaede = [
            Gebaeude {
                ge_name: String::from("Testgebäude 1"),
            },
            Gebaeude {
                ge_name: String::from("Testgebäude 2"),
            },
            Gebaeude {
                ge_name: String::from("Testgebäude 3"),
            },
        ];

        const ABC: [char; 10] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
        const ABC_LENGTH: usize = ABC.len();

        let mut raeume = Vec::new();
        let mut schraenke = Vec::new();
        for (gebauede_index, gebaeude) in all_gebaede.iter().enumerate() {
            for stockwerk in 0..3 {
                let schrank = Schrank {
                    sc_id: 0,
                    sc_ge_name: gebaeude.ge_name.clone(),
                    sc_nummer: String::from(ABC[(gebauede_index + stockwerk) % ABC_LENGTH]),
                    sc_stockwerk: stockwerk as i32,
                };
                schraenke.push(schrank);

                for raum_index in 0..10 {
                    let raum = Raum {
                        ra_id: 0,
                        ra_nummer: format!("{}", gebauede_index * 100 + raum_index),
                        ra_stockwerk: stockwerk as i32,
                        ra_ge_name: gebaeude.ge_name.clone(),
                    };
                    raeume.push(raum);
                }
            }
        }

        let mut dosen = Vec::new();
        for (raum_index, _) in raeume.iter().enumerate() {
            for dosen_index in 0..20 {
                let dose = Dose {
                    do_id: 0,
                    do_ra_id: raum_index as i32 + 1,
                    do_nummer: dosen_index.to_string(),
                    do_hat_telefon: dosen_index % 2 == 0,
                    do_hat_pc: dosen_index % 3 == 0,
                    do_hat_drucker: dosen_index % 5 == 0,
                };

                dosen.push(dose);
            }
        }

        let mut switches = Vec::new();
        for (schrank_index, schrank) in schraenke.iter().enumerate() {
            for switch_index in 0..3 {
                let switch = Switch {
                    sw_name: format!(
                        "{}_{}_{}",
                        schrank.sc_ge_name, schrank.sc_stockwerk, switch_index
                    ),
                    sw_sc_id: schrank_index as i32 + 1,
                    sw_ip: format!("{0}.{0}.{0}.{0}", schrank_index * 100 + switch_index),
                };
                switches.push(switch);
            }
        }

        let switches_length = switches.len();
        let mut switch_zu_dosen = Vec::new();
        for (dose_index, _) in dosen.iter().enumerate().step_by(3) {
            let switch = &switches[dose_index % switches_length];

            let vlan = match dose_index % 10 {
                0..=2 => 300,
                3 => 200,
                _ => 300,
            };

            let kommentar = if dose_index % 2 == 0 || dose_index % 3 == 0 {
                None
            } else {
                Some(String::from("Testkommentar"))
            };

            let switch_zu_dose = SwitchZuDose {
                szd_sw_name: switch.sw_name.clone(),
                szd_do_id: dose_index as i32 + 1,
                szd_port: format!("Fa0/{}", dose_index % switches_length),
                szd_vlan: vlan,
                szd_kommentar: kommentar,
            };

            switch_zu_dosen.push(switch_zu_dose);
        }

        println!("Inserting Gebäude...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for gebaeude in all_gebaede {
            sqlx::query("INSERT INTO ge_gebaeude VALUES ($1)")
                .bind(&gebaeude.ge_name)
                .execute(&mut *transaction)
                .await
                .unwrap();
        }
        transaction.commit().await.unwrap();

        println!("Inserting Räume...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for raum in raeume {
            sqlx::query("INSERT INTO ra_raum VALUES (NULL, $1, $2, $3)")
                .bind(&raum.ra_ge_name)
                .bind(&raum.ra_nummer)
                .bind(raum.ra_stockwerk)
                .execute(&mut *transaction)
                .await
                .unwrap();
        }
        transaction.commit().await.unwrap();

        println!("Inserting Schränke...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for schrank in schraenke {
            sqlx::query("INSERT INTO sc_schrank VALUES (NULL, $1, $2, $3)")
                .bind(&schrank.sc_ge_name)
                .bind(&schrank.sc_nummer)
                .bind(schrank.sc_stockwerk)
                .execute(&mut *transaction)
                .await
                .unwrap();
        }
        transaction.commit().await.unwrap();

        println!("Inserting Dosen...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for dose in dosen {
            sqlx::query("INSERT INTO do_dose VALUES (NULL, $1, $2, $3, $4, $5)")
                .bind(dose.do_ra_id)
                .bind(&dose.do_nummer)
                .bind(dose.do_hat_telefon)
                .bind(dose.do_hat_pc)
                .bind(dose.do_hat_drucker)
                .execute(&mut *transaction)
                .await
                .unwrap();
        }
        transaction.commit().await.unwrap();

        println!("Inserting Switches...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for switch in switches {
            sqlx::query("INSERT INTO sw_switch VALUES ($1, $2, $3)")
                .bind(&switch.sw_name)
                .bind(switch.sw_sc_id)
                .bind(&switch.sw_ip)
                .execute(&mut *transaction)
                .await
                .unwrap();
        }
        transaction.commit().await.unwrap();

        println!("Inserting SwitchZuDosen...");
        let mut transaction = self.connection_pool.begin().await.unwrap();
        for switch_zu_dose in switch_zu_dosen {
            sqlx::query("INSERT INTO szd_switch_zu_dose VALUES ($1, $2, $3, $4, $5)")
                .bind(&switch_zu_dose.szd_sw_name)
                .bind(switch_zu_dose.szd_do_id)
                .bind(&switch_zu_dose.szd_port)
                .bind(switch_zu_dose.szd_vlan)
                .execute(&mut *transaction)
                .await
                .unwrap();
        }
        transaction.commit().await.unwrap();
    }
}
