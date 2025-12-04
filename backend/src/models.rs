use argon2::password_hash::Salt;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct UserId {
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
#[serde_as]
pub struct DatabaseUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub firstname: String,
    pub password: String,
    pub middlename: Option<String>,
    pub lastname: String,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub created_at: Option<OffsetDateTime>,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateCurrencyRequest {
    pub currency_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
#[serde_as]
pub struct CreateUserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub created_at: Option<OffsetDateTime>,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct Currency {
    pub id: i32,
    pub currency_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub id: Option<i32>,
    pub currency_amount: Option<f32>,
    pub currency_code: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct UsernameCheckParams {
    pub name: String
}

#[derive(Deserialize)]
pub struct CurrencyByCodeParams {
    pub code: String
}

pub struct HashPasswordResult<'a> {
    pub salt: Salt<'a>,
    pub password_hash: String
}

impl<'a> HashPasswordResult<'a> {
    pub fn new(salt: Salt<'a>, password_hash: String) -> Self {
        Self { salt, password_hash }
    }
}

pub mod auth_responses {

    pub mod success_responses {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        pub struct LoggedInUser {
            token: String,
        }

        #[derive(Serialize)]
        pub struct UsernameAlreadyTaken {
            message: String
        }

        #[derive(Serialize)]
        pub struct UsernameAvailable {
            message: String
        }

        impl UsernameAvailable {
            pub fn new<S: Into<String>>(message: S) -> Self {
                Self {
                    message: message.into()
                }
            }
        }

        impl UsernameAlreadyTaken {
            pub fn new<S: Into<String>>(message: S) -> Self {
                Self {
                    message: message.into()
                }
            }
        }

        impl LoggedInUser {
            pub fn new<S: Into<String>>(token: S) -> Self {
                Self {
                    token: token.into(),
                }
            }
        }
    }
    pub mod error_responses {
        use serde::{Deserialize, Serialize};
        #[derive(Serialize, Deserialize)]
        pub struct UserNotFound {
            message: String,
        }

        #[derive(Serialize, Deserialize)]
        pub struct MissingAuthorizationHeader {
            message: String,
        }

        impl MissingAuthorizationHeader {
            pub fn new<S: Into<String>>(msg: S) -> Self {
                Self {
                    message: msg.into()
                }
            }
        }

        impl UserNotFound {
            pub fn new<S: Into<String>>(msg: S) -> Self {
                Self {
                    message: msg.into(),
                }
            }
        }
    }
}