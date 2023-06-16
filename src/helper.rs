use crate::database::connect_to_db::MongoDB;
use crate::database::FindUserBy;

use rocket::http::Status;
use bcrypt::hash;


//check valid text
pub fn check_valid_text(text: &str, max_size: usize, min_size: usize) -> bool {
    return if !text.is_empty() && text.len() <= max_size && text.len() >= min_size {
        true
    } else {
        false
    };
}

pub fn check_valid_name(text: &str, max_size: usize, min_size: usize) -> bool {
    return if text.is_empty() || text.len() <= max_size && text.len() >= min_size {
        true
    } else {
        false
    };
}

pub async fn find_user_by_login_and_mail(
    database: &MongoDB,
    mail: &str,
    nick_name: &str,
) -> FindUserBy {
    match database.find_user_by("mail", mail).await {
        Ok(None) => match database.find_user_by("data.user_nickname", nick_name).await {
            Ok(None) => FindUserBy::UserNotFound,
            Ok(Some(_)) => FindUserBy::UserFoundByNickName,
            Err(_) => FindUserBy::UserFoundByNickName,
        },
        Ok(Some(_)) => FindUserBy::UserFoundByEmail,
        Err(_) => FindUserBy::UserFoundByEmail,
    }
}

pub fn hash_text(text: String, cost: u32) -> Result<String, Status> {
    return match hash(text, cost) {
        Ok(hash_text) => Ok(hash_text),
        Err(_) => Err(Status::BadRequest),
    };
}
