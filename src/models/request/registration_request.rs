use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegistrationRequest{
    pub user_nickname: String,
    pub password: String,

    pub mail: String,

    pub first_name: String,
    pub last_name: String,
}
