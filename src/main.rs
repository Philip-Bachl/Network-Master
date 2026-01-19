use std::{
    env,
    process::exit,
    sync::{Arc, Mutex},
};

use dotenvy::dotenv;

use masterbase::Masterbase;
use rocket::{State, get, http::Status, launch, routes, serde::json::Json};

use crate::model::Gebaeude;

mod masterbase;
mod model;
mod schema;

struct MasterState {
    database: Arc<Mutex<Masterbase>>,
}

#[get("/gebaeude")]
async fn get_gebaeude_all(state: &State<MasterState>) -> Result<Json<Vec<Gebaeude>>, Status> {
    let Ok(mut database) = state.database.lock() else {
        println!("Poisioned Lock detected at 'get_gebaeude_all'");
        return Err(Status::InternalServerError);
    };

    let gebaeude = database.read_gebaeude_all();

    Ok(Json(gebaeude))
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

    rocket::build()
        .manage(MasterState {
            database: Arc::new(Mutex::new(database)),
        })
        .mount("/db", routes![get_gebaeude_all])
}
