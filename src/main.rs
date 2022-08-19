mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
//add imports below
use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users};
use api::example_simple_api::{index_view, hello_view, template_file_view, user_agent_view};
use api::example_products_api::{new_product_view, get_product_view};
use repository::mongodb_repo::MongoRepo;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db = MongoRepo::init();
    let _rocket = rocket::build()
        .manage(db)

        // Examples
        .mount("/", routes![index_view])
        .mount("/", routes![hello_view])
        .mount("/", routes![template_file_view])
        .mount("/check", routes![user_agent_view])

        .mount("/", routes![new_product_view])
        .mount("/", routes![get_product_view])

        // Static files
        .mount("/static", FileServer::from("static/"))

        // CRUD[mongodb + users]
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
        .launch()
        .await?;

    Ok(())
}
