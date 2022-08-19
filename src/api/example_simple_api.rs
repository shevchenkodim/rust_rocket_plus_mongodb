// Methods
// #[any_method_name("/")] -> get, put, post, delete, head, patch, options

use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use crate::api::custom_from_request::user_agent_from_request::UserAgent;

#[get("/")]
pub async fn index_view() -> &'static str {
    "Home page"
}

#[get("/user-agent")]
pub async fn user_agent_view(info: UserAgent<'_>) -> String {
    format!("User-Agent: {:?}", info)
}

#[get("/hello/<name>/<age>/<cool>")]
pub async fn hello_view(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/templates/<file..>")]
pub async fn template_file_view(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/").join(file)).await.ok()
}
