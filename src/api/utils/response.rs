use serde::{Serialize, Deserialize};
use rocket::serde::json::{serde_json, Value};
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::{ContentType, Status};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub payload: Option<Value>,
    pub error: Option<String>
}


impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to<'json>(self, req: &'r Request<'_>) -> response::Result<'static> {
        let mut status_code = Status::Ok;
        if !self.success {
            status_code = Status::BadRequest;
        }

        let serialized = serde_json::to_string(&self).unwrap();
        Response::build_from(serialized.respond_to(req)?)
            .header(ContentType::new("application", "json"))
            .status(status_code)
            .ok()
    }
}


pub fn build_response(success: bool, payload: Option<Value>, error: Option<String>) -> ApiResponse {
    ApiResponse {
        success,
        payload,
        error
    }
}
