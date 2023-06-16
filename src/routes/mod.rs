pub mod user_data;
pub mod token;
pub mod validator_authorization;

pub enum TypeValidDataFromRegistration {
    Ok,
    BadFirstName,
    BadLastName,
    BadLogin,
    BadPassword,
    BadMail,
    BadNickName,
}

pub enum TypeValidTwoStr {
    Ok,
    BadFirst,
    BadSecond,
    BadThree,
}

pub enum TypeValidMail {
    Ok,
    BadMail,
}