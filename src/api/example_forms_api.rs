use rocket::form::{Form, Strict};
use rocket::fs::TempFile;

use rocket::time::Date;
use rocket::form::{self, Error};

#[derive(FromForm)]
struct CreditCard {
    #[field(validate = luhn(self.cvv, &self.expiration))]
    number: u64,
    #[field(validate = range(..9999))]
    cvv: u16,
    expiration: Date,
}

fn luhn<'v>(number: &u64, cvv: u16, exp: &Date) -> form::Result<'v, ()> {
    if !valid {
        Err(Error::validation("invalid credit card number"))?;
    }
    Ok(())
}

#[derive(FromForm)]
struct Task<'r> {
    complete: bool,
    r#type: &'r str,
}

#[derive(FromForm)]
struct Upload<'r> {
    save: bool,
    file: TempFile<'r>,
}

#[derive(FromForm)]
struct Input {
    required: Strict<bool>,
    uses_default: bool
}


#[derive(FromForm)]
struct ExternalOne<'r> {
    #[field(name = "first-Name")]
    first_name: &'r str
}

#[derive(FromForm)]
struct ExternalTwo<'r> {
    #[field(name = uncased("firstName"))]
    #[field(name = "first_name")]
    first_name: &'r str
}


#[post("/todo", data = "<task>")]
fn new(task: Form<Task<'_>>) { /* .. */ }

#[post("/upload", data = "<upload>")]
fn upload_form(upload: Form<Upload<'_>>) { /* .. */ }
