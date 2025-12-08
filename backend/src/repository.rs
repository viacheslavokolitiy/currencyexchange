use sqlx::PgPool;

pub struct Repository {
    pub pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[cfg(test)]
mod repository_spec {
    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

    #[test]
    fn should_verify_pwds() {
        let password = "qwerty12345";
        let password_bytes = password.as_bytes();
        let argon2 = Argon2::default();
        let salt = SaltString::generate();
        let hashed = argon2.hash_password(password_bytes, &salt)
            .unwrap()
            .to_string();
        println!("Hashed password{}", hashed);

        let parsed_hash = PasswordHash::new(hashed.as_str());
        assert!(parsed_hash.is_ok());

        let x = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash.unwrap())
            .is_ok();
        assert_eq!(x, true);
    }
}