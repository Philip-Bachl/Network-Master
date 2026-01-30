use std::{env, fs, process::exit};

use dotenvy::dotenv;

use rocket::{fs::FileServer, launch, routes};

use masterbase::Masterbase;

mod endpoints;
mod error;
mod masterbase;
mod model;

#[launch]
async fn rocket() -> _ {
    if dotenv().is_err() {
        println!("INITERROR: Could not find .env file!");
        exit(1);
    };

    let Ok(connection_string) = env::var("DATABASE_URL") else {
        println!("INITERROR: Could not retrieve \"DATABASE_URL\" from .env file!");
        exit(2);
    };
    let Ok(masterbase) = Masterbase::init(&connection_string).await else {
        println!(
            "INITERROR: Could not establish connection with given database url: \"{connection_string}\"!"
        );
        exit(3);
    };

    if Some("--reset".to_string()) == env::args().nth(1) {
        for (i, line) in include_str!("../db_scripts/down.sql")
            .split_inclusive(';')
            .chain(include_str!("../db_scripts/up.sql").split_inclusive(';'))
            .enumerate()
        {
            sqlx::query(line)
                .execute(&masterbase.connection_pool)
                .await
                .unwrap();
            println!("Running reset query line number {i}: {line}");
        }
    }

    rocket::build()
        .manage(masterbase)
        .mount(
            "/api",
            routes![
                endpoints::gebaeude::create_gebaede,
                endpoints::gebaeude::read_gebaeude_all,
                endpoints::gebaeude::update_gebaeude,
                endpoints::gebaeude::delete_gebaeude,
                //
                endpoints::raum::create_raum,
                endpoints::raum::read_raeume_all,
                endpoints::raum::update_raum,
                endpoints::raum::delete_raum,
                //
                endpoints::schrank::create_schrank,
                endpoints::schrank::read_schrank_all,
                endpoints::schrank::update_schrank,
                endpoints::schrank::delete_schrank,
            ],
        )
        .mount("/", FileServer::from("./static"))
}
