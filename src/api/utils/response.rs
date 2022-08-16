use serde::{Serialize, Deserialize};
// use rocket::{serde::json::Json};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub payload: Option<String>,
    pub error: Option<String>
}

impl ApiResponse {
    // pub fn json(&self) -> Json<ApiResponse> {
    //     Json(self)
    // }
}

pub fn build_response(success: bool, payload: Option<String>, error: Option<String>) -> ApiResponse {
    ApiResponse {
        success,
        payload,
        error
    }
}
