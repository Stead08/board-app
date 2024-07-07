use crate::value_object::{Content, Email, HashedPassword, Name, Password, PostId, Title, UserId};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: Name,
    pub email: Email,
    pub password: HashedPassword,
}
#[derive(Debug, Clone)]
pub struct Post {
    pub id: PostId,
    pub user_id: UserId,
    pub title: Title,
    pub content: Content,
}

#[derive(Debug, Error)]
pub enum HashPasswordError {
    #[error("Failed to hash password")]
    HashPassword,
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Failed to create user")]
    CreateUser,
}

impl User {
    pub fn new(
        id: UserId,
        name: Name,
        email: Email,
        password: Password,
    ) -> Result<Self, UserError> {
        let password = hash_password(password).map_err(|_e| UserError::CreateUser)?;
        Ok(Self {
            id,
            name,
            email,
            password,
        })
    }
}

fn hash_password(password: Password) -> Result<HashedPassword, HashPasswordError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_e| HashPasswordError::HashPassword)?
        .to_string();
    Ok(HashedPassword::from(password_hash))
}
