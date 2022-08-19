use rocket::http::{Status, Header, ContentType};
use rocket::response::{content, status};

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct RawSuccessJson(&'static str);


#[get("/response/1")]
pub async fn get_response_1() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::JSON, "{ \"hi\": \"world\" }"))
}


#[get("/response/2")]
pub async fn get_response_2() -> RawSuccessJson {
    RawSuccessJson("{ \"hi\": \"world\" }")
}




