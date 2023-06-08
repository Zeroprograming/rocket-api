#[macro_use]
extern crate rocket;

// macro use rocket
use rocket::get;
use rocket::http::Method;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};

// Constans and database
use crate::error_response::error_responses::{
    ErrorResponse, NOT_FOUND_JSON, UNAUTHORIZED_JSON, UNKNOWN_JSON,
};
use crate::constants::{UNAUTHORIZED, UNKNOWN};
use crate::database::connect_to_db::init;


// Add the modules
pub mod error_response;
pub mod constants;
mod routes;
mod models;
mod private;
mod database;


#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Welcome to my API from Rocket.rs")))
}

#[launch]
async fn rocket() -> _ {
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);

    println!("hello world!, <starting REST API>");

    rocket::build()
        //Get routes
        .attach(init().await)
        .mount(
            "/",
            routes![
                routes::user_data::datauser,
                hello,
                routes::hola,
                routes::hello,
                routes::hello_put,
                routes::hello_post,
                routes::hello_delete
            ],
        )
        .manage(cors.to_cors())
        .register(
            "/",
            catchers![unauthorized, not_found, internal_sever_error,],
        )
}

#[catch(401)]
pub fn unauthorized() -> Json<ErrorResponse> {
    Json(UNAUTHORIZED_JSON)
}

#[catch(404)]
pub fn not_found() -> Json<ErrorResponse> {
    Json(NOT_FOUND_JSON)
}

#[catch(500)]
pub fn internal_sever_error() -> Json<ErrorResponse> {
    Json(UNKNOWN_JSON)
}