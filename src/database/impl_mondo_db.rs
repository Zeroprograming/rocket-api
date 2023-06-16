use crate::database::connect_to_db::MongoDB;
use crate::database::{FindUserBy, RegistrationError, LoginError};

use crate::helper::{find_user_by_login_and_mail, hash_text};
use crate::routes::token::create_token::encode_token_and_refresh;
use crate::models::model_user::{Dataplayer, User};
use crate::models::request::registration_request::RegistrationRequest;
use crate::models::request::login_request::LoginRequest;

use bcrypt::verify;
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
        let collection_user = self.database.collection::<User>("users");
        println!("Error: {}", data_find_in);
        Ok(collection_user
            .find_one(bson::doc! { find_by: data_find_in }, None)
            .await?)
    }

    pub async fn login(
        &self,
        login_request: Json<LoginRequest>,
    ) -> mongodb::error::Result<LoginError> {
        match Self::find_user_by(self, "mail", &login_request.login).await {
            Ok(option_user) => match option_user {
                None => match Self::find_user_by(self, "data.user_nickname", &login_request.login).await {
                    Ok(option_data_user) => match option_data_user {
                        None => Ok(LoginError::WrongLogin),
                        Some(user) => match verify(&login_request.password, &user.password) {
                            Ok(true) => {
                                match encode_token_and_refresh(user._id.clone(), JWT_SECRET, REFRESH_JWT_SECRET, EXPIRATION_REFRESH_TOKEN, EXPIRATION_TOKEN){
                                    Ok(token) => Ok(LoginError::Ok(token)),
                                    Err(_) => Ok(LoginError::Unknown),
                                }
                            }
                            Ok(false) => Ok(LoginError::WrongPassword),
                            Err(_) => Ok(LoginError::Unknown),
                        },
                    },
                    Err(_) => Ok(LoginError::Unknown),
                },
                Some(user) => match verify(&login_request.password, &user.password) {
                    Ok(true) => {
                        match encode_token_and_refresh(user._id.clone(), JWT_SECRET, REFRESH_JWT_SECRET, EXPIRATION_REFRESH_TOKEN, EXPIRATION_TOKEN){
                            Ok(token) => Ok(LoginError::Ok(token)),
                            Err(_) => Ok(LoginError::Unknown),
                        }
                    }
                    Ok(false) => Ok(LoginError::WrongPassword),
                    Err(_) => Ok(LoginError::Unknown),
                },
            },
            Err(_) => Ok(LoginError::Unknown),                
        }
    }

    pub async fn registration(
        &self,
        registration_request: Json<RegistrationRequest>,
    ) -> mongodb::error::Result<RegistrationError> {
        let collection_user = self.database.collection::<User>("users");
        match find_user_by_login_and_mail(
            self,
            &registration_request.mail,
            & registration_request.user_nickname,
        )
        .await
        {
            FindUserBy::UserNotFound => match hash_text(registration_request.password.clone(), 4) {
                Ok(hash_password) => {
                    // Defino usuario apartir de la base de datos de model User para el modelo de User y el modelo de Dataplayer
                    let user = User {
                        _id: ObjectId::new(),
                        user_id: ObjectId::new().to_string(),
                        password: hash_password,
                        mail: registration_request.mail.to_string(),
                        first_name: registration_request.first_name.clone(),
                        last_name: registration_request.last_name.clone(),
                        data: Dataplayer{
                            user_score: 0,
                            user_avatar_id: "".to_string(),
                            user_avatar_url: "".to_string(),
                            user_nickname: registration_request.user_nickname.clone(),
                        },
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
            FindUserBy::UserFoundByNickName => Ok(RegistrationError::AlreadyRegisteredByNickName),
        }
    }
}
