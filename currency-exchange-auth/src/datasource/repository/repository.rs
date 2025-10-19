use crate::datasource::api_models::CreateUserRequest;
use crate::datasource::errors::DataError;
use crate::datasource::models::User;
use crate::datasource::repository::user_repository::UserRepository;
use sqlx::PgPool;
use time::OffsetDateTime;

pub struct Repository {
    pool: PgPool
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for Repository {
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.pool)
            .await
            .expect("Error loading user");
        Ok(result)
    }

    async fn create_user(&self, user: &CreateUserRequest) -> Result<Option<User>, DataError> {
        let find_user_result = self.find_user_by_username(&user.username).await.expect("Error loading user");
        if find_user_result.is_none() {
            let username = &user.username;
            let email = &user.email;
            let password = &user.password;
            let firstname = &user.firstname;
            let middlename = &user.middlename;
            let lastname = &user.lastname;
            let created_at = OffsetDateTime::now_utc();
            let updated_at = OffsetDateTime::now_utc();
            let create_result = sqlx::query_as!(User,
                "INSERT INTO users(username, email, password, firstname, middlename, lastname, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
                username, email, password, firstname, middlename.clone().unwrap(), lastname, created_at, updated_at
            ).fetch_optional(&self.pool)
                .await
                .expect("Error creating user");
            Ok(create_result)
        } else {
            Err(DataError::EntryAlreadyExists(format!("User with name={} already exists", &user.username)))
        }
    }
}