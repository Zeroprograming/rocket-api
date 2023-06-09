use crate::database::connect_to_db::MongoDB;
use crate::database::{FindUserBy, RegistrationError};

use crate::helper::{find_user_by_login_and_mail, hash_text};
use crate::routes::token::create_token::encode_token_and_refresh;
use crate::models::model_user::User;
use crate::models::request::registration_request::RegistrationRequest;

use mongodb::bson::oid::ObjectId;
use mongodb::{bson, Database};
use rocket::serde::json::Json;

use crate::constants::{EXPIRATION_REFRESH_TOKEN, EXPIRATION_TOKEN};
use crate::private::{JWT_SECRET, REFRESH_JWT_SECRET};

impl MongoDB {
    pub fn new(database: Database) -> Self {
        MongoDB { database }
    }

    pub async fn find_user_by(
        &self,
        find_by: &str,
        data_find_in: &str,
    ) -> mongodb::error::Result<Option<User>> {
        let collection_user = self.database.collection::<User>("user");

        Ok(collection_user
            .find_one(bson::doc! { find_by: data_find_in }, None)
            .await?)
    }

    pub async fn registration(
        &self,
        registration_request: Json<RegistrationRequest>,
    ) -> mongodb::error::Result<RegistrationError> {
        let collection_user = self.database.collection::<User>("user");
        match find_user_by_login_and_mail(
            self,
            &registration_request.mail,
            &registration_request.login,
        )
        .await
        {
            FindUserBy::UserNotFound => match hash_text(registration_request.password.clone(), 4) {
                Ok(hash_password) => {
                    let user = User {
                        _id: ObjectId::new(),
                        login: registration_request.login.clone(),
                        password: hash_password,
                        mail: registration_request.mail.to_string(),
                        first_name: registration_request.first_name.clone(),
                        last_name: registration_request.last_name.clone(),
                    };
                    collection_user.insert_one(&user, None).await?;
                    match encode_token_and_refresh(
                        user._id.clone(),
                        JWT_SECRET,
                        REFRESH_JWT_SECRET,
                        EXPIRATION_REFRESH_TOKEN,
                        EXPIRATION_TOKEN,
                    ) {
                        Ok(tokens) => Ok(RegistrationError::Ok(tokens)),
                        Err(_) => Ok(RegistrationError::Unknown),
                    }
                }
                Err(_) => Ok(RegistrationError::WrongPassword),
            },
            FindUserBy::UserFoundByEmail => Ok(RegistrationError::AlreadyRegisteredByEmail),
            FindUserBy::UserFoundByLogin => Ok(RegistrationError::AlreadyRegisteredByLogin),
        }
    }
}
