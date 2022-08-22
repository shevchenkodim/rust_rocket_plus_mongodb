// Methods
// #[any_method_name("/")] -> get, put, post, delete, head, patch, options
use std::time::{SystemTime, UNIX_EPOCH};
// use std::fmt::format;
use std::path::{Path, PathBuf};
use rocket::fs::{NamedFile, TempFile};
// use rocket::http::ContentType;
use rocket::Request;
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

#[post("/file/upload", data = "<file>")]
pub async fn file_upload_view(mut file: TempFile<'_>) -> std::io::Result<()> {
    let content_type = file.content_type().unwrap().to_string();
    let f_ext = content_type.split("/").last().unwrap().to_string();
    let f_name = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    file.persist_to(String::from(format!("tmp/{}.{}", f_name, f_ext))).await?;
    Ok(())
}

#[catch(404)]
pub async fn not_found(req: &Request<'_>) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}
