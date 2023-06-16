use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub user_id: String,

    pub mail: String, 
    pub password: String,

    pub first_name: String,
    pub last_name: String,
    
    pub data: Dataplayer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataplayer {
    pub user_score: i32,
    
    pub user_nickname: String,

    pub user_avatar_id: String,

    pub user_avatar_url: String,

}
