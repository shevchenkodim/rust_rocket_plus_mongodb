use rocket::http::{CookieJar, Cookie, CookieJar};
use rocket::response::{Flash, Redirect};

#[get("/cookies/get")]
pub async fn cookies_view(cookies: &CookieJar<'_>) -> Option<String> {
    cookies.get("some_key").map(|crumb| format!("Key: {}", crumb.value()))
}

#[get("/cookies/get_private")]
pub async fn cookies_get_view(cookies: &CookieJar<'_>) -> Option<String> {
    cookies.get_private("some_private_key").map(|crumb| format!("Key: {}", crumb.value()))
}

#[get("/cookies/set_private")]
pub async fn cookies_set_view(cookies: &CookieJar<'_>) -> Option<String> {
    cookies.add_private(Cookie::new("some_private_key", "msg"));
    format!("Success: {}", true)
}

#[post("/cookies/remove_private")]
pub async fn remove_private_view(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("some_private_key"));
    Flash::success(Redirect::to("/"), "Success: true")
}

