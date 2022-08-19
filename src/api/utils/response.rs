use serde::{Serialize, Deserialize};
use rocket::serde::json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub payload: Value,
    pub error: Option<String>
}

pub fn build_response(success: bool, payload: Value, error: Option<String>) -> ApiResponse {
    ApiResponse {
        success,
        payload,
        error
    }
}
