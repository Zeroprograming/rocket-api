use rocket::{get, http::Status, serde::json::Json};

#[get("/datauser")]
pub fn datauser() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Message from folder data")))
}