use std::{
    env,
    process::exit,
    sync::{Arc, Mutex},
};

use dotenvy::dotenv;

use masterbase::Masterbase;
use rocket::{State, get, http::Status, launch, post, routes, serde::json::Json};

use crate::model::Gebaeude;

mod masterbase;
mod model;
mod schema;

struct MasterState {
    database: Arc<Mutex<Masterbase>>,
}

//TODO: error handling with custom type

#[get("/gebaeude")]
async fn get_gebaeude_all(state: &State<MasterState>) -> Result<Json<Vec<Gebaeude>>, Status> {
    let Ok(mut database) = state.database.lock() else {
        println!("ERROR: Poisioned Lock detected at 'get_gebaeude_all'");
        return Err(Status::InternalServerError);
    };

    let gebaeude_all = match database.read_gebaeude_all() {
        Ok(ge_al) => ge_al,
        Err(err) => {
            println!("ERROR: at 'get_gebaeude_all': {err}");
            return Err(Status::InternalServerError);
        }
    };

    Ok(Json(gebaeude_all))
}

//TODO: add read endpoints with filters

#[post("/gebaeude", data = "<gebaeude>")]
async fn add_gebaede(state: &State<MasterState>, gebaeude: Json<Gebaeude>) -> Status {
    let Ok(mut database) = state.database.lock() else {
        println!("ERROR: Poisioned Lock detected at 'add_gebaeude'");
        return Status::InternalServerError;
    };

    let gebaeude = gebaeude.0;

    match database.create_gebaeude(gebaeude) {
        Ok(_) => Status::Created,
        Err(err) => {
            println!("ERROR: at 'add_gebaeude': {err}");
            Status::InternalServerError
        }
    }
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
        .mount("/db", routes![get_gebaeude_all, add_gebaede]);

    for r in app.routes() {
        println!("{r}");
    }

    app
}
