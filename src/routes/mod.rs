use rocket::{get, http::Status, serde::json::Json};

pub mod user_data;
pub mod token;
pub mod validator_authorization;

#[get("/hola")]
pub fn hola() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongoDB")))
}

pub enum TypeValidDataFromRegistration {
    Ok,
    BadFirstName,
    BadLastName,
    BadLogin,
    BadPassword,
    BadMail,
}

pub enum TypeValidTwoStr {
    Ok,
    BadFirst,
    BadSecond,
}

pub enum TypeValidMail {
    Ok,
    BadMail,
}