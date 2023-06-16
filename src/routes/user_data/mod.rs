use rocket::serde::json::Json;
use crate::models::request::registration_request::RegistrationRequest;
use crate::models::request::login_request::LoginRequest;


pub mod registration;
pub mod login;

pub enum RegistrationRequestError {
    Ok(Json<RegistrationRequest>),
    NoneRegistrationRequest,
    BadFirstName,
    BadLastName,
    BadLogin,
    BadPassword,
    BadMail,
    BadNickName,
}

pub enum LoginRequestError{
    Ok(Json<LoginRequest>),
    NoneLoginRequest,
    BadLogin,
    BadPassword,
}