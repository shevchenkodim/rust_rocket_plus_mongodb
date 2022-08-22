mod api;
mod models;
mod config;
mod repository;

#[macro_use]
extern crate rocket;

use std::env;
use std::path::PathBuf;
use rocket::fs::FileServer;
//add imports below
use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users};
use api::example_simple_api::{index_view, hello_view, template_file_view, user_agent_view,
                              file_upload_view, not_found};
use api::example_products_api::{new_product_view, get_product_view};
use api::example_auth_api::{check_auth_view, login_auth_view};
use repository::mongodb_repo::MongoRepo;



#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let temp_dir: PathBuf = env::temp_dir();
    let db = MongoRepo::init();

    let _rocket = rocket::build()
        .manage(db)
        .attach(config::AppState::manage())

        // Examples
        // auth
        .mount("/", routes![check_auth_view])
        .mount("/", routes![login_auth_view])

        .mount("/", routes![index_view])
        .mount("/", routes![hello_view])
        .mount("/", routes![template_file_view])
        .mount("/check", routes![user_agent_view])
        .mount("/", routes![file_upload_view])

        .mount("/", routes![new_product_view])
        .mount("/", routes![get_product_view])

        // CRUD[mongodb + users]
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])

        // Static files
        .mount("/static", FileServer::from("static/"))

        // System
        .register("/", catchers![not_found])

        .launch()
        .await?;

    Ok(())
}
