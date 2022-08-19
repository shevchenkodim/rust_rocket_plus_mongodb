use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

#[derive(Debug)]
pub struct UserAgent<'r>(&'r str);

#[derive(Debug)]
pub enum UserAgentError {
    Undefined,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgent<'r> {
    type Error = UserAgentError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("User-Agent") {
            None => Outcome::Failure((Status::BadRequest, UserAgentError::Undefined)),
            Some(user_agent) => Outcome::Success(UserAgent(user_agent)),
        }
    }
}