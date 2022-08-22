use rocket::serde::json::{Value};
use rocket::serde::json::serde_json::json;
use rocket::State;
use crate::api::auth::{Auth, generate_new_session};
use crate::config::AppState;

#[get("/check_auth")]
pub async fn check_auth_view(auth: Auth) -> &'static str {
    println!("{}", auth.exp);
    println!("{}", auth.session_id);
    "Home page"
}


#[post("/login")]
pub async fn login_auth_view(state: &State<AppState>) -> Value {
    let token = generate_new_session(&state.secret);
    json!({"token": token})
}
