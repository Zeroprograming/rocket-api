use rocket::{get, http::Status, serde::json::Json};
use crate::models::request::registration_request::RegistrationRequest;


pub mod registration;

#[get("/datauser")]
pub fn datauser() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Message from folder data")))
}

pub enum RegistrationRequestError {
    Ok(Json<RegistrationRequest>),
    NoneRegistrationRequest,
    BadFirstName,
    BadLastName,
    BadLogin,
    BadPassword,
    BadMail,
}