use mongodb::bson::DateTime;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: Option<i32>,
    pub name: String,
    pub code: String,
    pub created_at: Option<String>
}


#[post("/product", format = "application/json", data = "<product>")]
pub async fn new_product_view(product: Json<Product>) -> Result<Json<bool>, Status> {
    println!("{:?}", product);
    Ok(Json(true))
}

#[get("/product/<id>", format = "json")]
pub async fn get_product_view(id: i32) -> Result<Json<Product>, Status> {
    Ok(Json(Product {
        id: Some(id),
        name: String::from("name"),
        code: String::from("code"),
        created_at: Option::from(DateTime::now().to_string()),
    }))
}
