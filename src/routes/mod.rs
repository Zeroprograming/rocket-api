use rocket::{get,post,put,delete, http::Status, serde::json::Json};

pub mod user_data;

#[get("/hola")]
pub fn hola() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongoDB")))
}


#[get("/hello/<name>")]
pub fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[put("/hello/<name>")]
pub fn hello_put(name: &str) -> String {
    format!("Hello, {}!", name)
}

// ADD POST METHOD HTTP
#[post("/hello/<name>")]
pub fn hello_post(name: &str) -> String {
    format!("Hello, {}!", name)
}

// ADD DELETE METHOD HTTP
#[delete("/hello/<name>")]
pub fn hello_delete(name: &str) -> String {
    format!("Hello, {}!", name)
}