use sqlx::{Pool, Sqlite, sqlite::SqliteConnectOptions};

use crate::{
    endpoints::{gebaeude, schrank, switch},
    error::Error,
    model::{Dose, Gebaeude, Raum, Schrank, Switch, SwitchZuDose},
};

//mod gebaeude;
//mod raum;

pub struct Masterbase {
    pub connection_pool: Pool<Sqlite>,
}

impl Masterbase {
    pub async fn init(connection_string: &str) -> Result<Masterbase, Error> {
        let connection_pool = Pool::connect_lazy_with(
            SqliteConnectOptions::new()
                .filename(connection_string)
                .create_if_missing(true),
        );

        Ok(Masterbase { connection_pool })
    }

    pub async fn seed(&self) {
        let gebaede = [
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

        let mut raeume = Vec::new();
        let mut schraenke = Vec::new();
        for (i, geb) in gebaede.iter().enumerate() {
            for j in 0..3 {
                let schrank = Schrank {
                    sc_nummer: String::from(ABC[i + j]),
                    sc_stockwerk: j as i32,
                    sc_ge_name: geb.ge_name.clone(),
                };
                schraenke.push(schrank);

                for k in 0..10 {
                    let raum = Raum {
                        ra_nummer: format!("{}", j * 100 + k),
                        ra_stockwerk: j as i32,
                        ra_ge_name: geb.ge_name.clone(),
                    };
                    raeume.push(raum);
                }
            }
        }

        let mut dosen = Vec::new();
        for (i, raum) in raeume.iter().enumerate() {
            for j in 0..20 {
                let dose = Dose {
                    do_nummer: j.to_string(),
                    do_ra_nummer: raum.ra_nummer.clone(),
                    do_ra_stockwerk: raum.ra_stockwerk,
                    do_ra_ge_name: raum.ra_ge_name.clone(),
                    do_hat_telefon: i % 2 == 0,
                    do_hat_pc: i % 3 == 0,
                    do_hat_drucker: i % 5 == 0,
                };

                dosen.push(dose);
            }
        }

        let mut switches = Vec::new();
        for (i, schrank) in schraenke.iter().enumerate() {
            let switch1 = if i % 3 != 0 {
                Switch {
                    sw_ip: format!("{i}.{i}.{i}.{i}"),
                    sw_sc_nummer: Some(schrank.sc_nummer.clone()),
                    sw_sc_stockwerk: Some(schrank.sc_stockwerk),
                    sw_sc_ge_name: Some(schrank.sc_ge_name.clone()),
                }
            } else {
                Switch {
                    sw_ip: format!("{i}.{i}.{i}.{i}"),
                    sw_sc_nummer: None,
                    sw_sc_stockwerk: None,
                    sw_sc_ge_name: None,
                }
            };
            let switch1 = if i % 3 != 0 {
                Switch {
                    sw_ip: format!("{0}.{0}.{0}.{0}", i + 1),
                    sw_sc_nummer: Some(schrank.sc_nummer.clone()),
                    sw_sc_stockwerk: Some(schrank.sc_stockwerk),
                    sw_sc_ge_name: Some(schrank.sc_ge_name.clone()),
                }
            } else {
                Switch {
                    sw_ip: format!("{0}.{0}.{0}.{0}", i + 1),
                    sw_sc_nummer: None,
                    sw_sc_stockwerk: None,
                    sw_sc_ge_name: None,
                }
            };
            switches.push(switch1);
        }

        let switches_length = switches.len();
        let mut switch_zu_dosen = Vec::new();
        for (i, dose) in dosen.iter().enumerate().step_by(3) {
            let switch_index = i % switches_length;
            let switch = &switches[switch_index];

            let switch_zu_dose = SwitchZuDose {
                sd_do_nummer: dose.do_nummer.clone(),
                sd_do_ra_nummer: dose.do_ra_nummer.clone(),
                sd_do_ra_stockwerk: dose.do_ra_stockwerk,
                sd_do_ra_ge_name: dose.do_ra_ge_name.clone(),
                sd_sw_ip: switch.sw_ip.clone(),
                sd_switchport: format!("Fa0/{}", i / 2),
                sd_kommentar: None,
            };
            switch_zu_dosen.push(switch_zu_dose);
        }

        println!("Inserting Gebäude...");
        for geb in gebaede {
            sqlx::query("INSERT INTO ge_gebaeude VALUES ($1)")
                .bind(&geb.ge_name)
                .execute(&self.connection_pool)
                .await
                .unwrap();
        }

        println!("Inserting Räume...");
        for raum in raeume {
            sqlx::query(
                "INSERT INTO ra_raum (ra_nummer, ra_stockwerk, ra_ge_name) VALUES ($1, $2, $3)",
            )
            .bind(&raum.ra_nummer)
            .bind(raum.ra_stockwerk)
            .bind(&raum.ra_ge_name)
            .execute(&self.connection_pool)
            .await
            .unwrap();
        }

        println!("Inserting Schränke...");
        for schrank in schraenke {
            sqlx::query("INSERT INTO sc_schrank VALUES ($1, $2, $3)")
                .bind(&schrank.sc_nummer)
                .bind(schrank.sc_stockwerk)
                .bind(&schrank.sc_ge_name)
                .execute(&self.connection_pool)
                .await
                .unwrap();
        }

        println!("Inserting Dosen...");
        for dose in dosen {
            sqlx::query("INSERT INTO do_dose VALUES ($1, $2, $3, $4, $5, $6, $7)")
                .bind(&dose.do_nummer)
                .bind(&dose.do_ra_nummer)
                .bind(dose.do_ra_stockwerk)
                .bind(&dose.do_ra_ge_name)
                .bind(dose.do_hat_telefon)
                .bind(dose.do_hat_pc)
                .bind(dose.do_hat_drucker)
                .execute(&self.connection_pool)
                .await
                .unwrap();
        }

        println!("Inserting Switches...");
        for switch in switches {
            sqlx::query("INSERT INTO sw_switch VALUES ($1, $2, $3, $4)")
                .bind(&switch.sw_ip)
                .bind(&switch.sw_sc_nummer)
                .bind(switch.sw_sc_stockwerk)
                .bind(&switch.sw_sc_ge_name)
                .execute(&self.connection_pool)
                .await
                .unwrap();
        }

        println!("Inserting SwitchZuDosen...");
        for switch_zu_dose in switch_zu_dosen {
            sqlx::query("INSERT INTO sd_switch_zu_dose VALUES ($1, $2, $3, $4, $5, $6, $7)")
                .bind(&switch_zu_dose.sd_do_nummer)
                .bind(&switch_zu_dose.sd_do_ra_nummer)
                .bind(switch_zu_dose.sd_do_ra_stockwerk)
                .bind(&switch_zu_dose.sd_do_ra_ge_name)
                .bind(&switch_zu_dose.sd_sw_ip)
                .bind(&switch_zu_dose.sd_switchport)
                .bind(&switch_zu_dose.sd_kommentar)
                .execute(&self.connection_pool)
                .await
                .unwrap();
        }
    }
}
