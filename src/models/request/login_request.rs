use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest{

    pub login: String,
    pub password: String,
    
}
