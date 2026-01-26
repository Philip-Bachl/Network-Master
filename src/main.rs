use std::{
    env,
    process::exit,
    sync::{Arc, Mutex},
};

use dotenvy::dotenv;

use masterbase::Masterbase;
use rocket::{launch, routes};

mod endpoints;
mod masterbase;
mod model;
mod schema;

struct MasterState {
    database: Arc<Mutex<Masterbase>>,
}

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
    let database = match Masterbase::connect(&connection_string) {
        Ok(db) => db,
        Err(err) => {
            println!("INITERROR: Could not connect to Database: {}", err);
            exit(3);
        }
    };

    let app = rocket::build()
        .manage(MasterState {
            database: Arc::new(Mutex::new(database)),
        })
        .mount(
            "/db",
            routes![
                endpoints::get_gebaeude_all,
                endpoints::add_gebaede,
                endpoints::change_gebaede
            ],
        );

    for r in app.routes() {
        println!("{r}");
    }

    app
}
