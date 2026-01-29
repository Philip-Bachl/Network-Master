use std::{env, process::exit};

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

    rocket::build()
        .manage(masterbase)
        .mount(
            "/api",
            routes![
                endpoints::gebaeude::create_gebaede,
                endpoints::gebaeude::read_gebaeude_all,
                endpoints::gebaeude::update_gebaeude,
                endpoints::gebaeude::delete_gebaeude,
            ],
        )
        .mount("/", FileServer::from("./static"))
}
