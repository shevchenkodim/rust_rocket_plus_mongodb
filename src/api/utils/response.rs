use serde::{Serialize, Deserialize};
// use rocket::{serde::json::Json};
use rocket::serde::json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub payload: Value,
    pub error: Option<String>
}

impl ApiResponse {
    // pub fn json(&self) -> Json<ApiResponse> {
    //     Json(self)
    // }
}

pub fn build_response(success: bool, payload: Value, error: Option<String>) -> ApiResponse {
    ApiResponse {
        success,
        payload,
        error
    }
}
