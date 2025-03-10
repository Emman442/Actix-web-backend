use crate::error::ErrorMessage;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordHash, PasswordVerifier, SaltString, rand_core::OsRng},
};
const MAX_PASSWORD_LENGTH: usize = 64;

pub fn hash(password: impl Into<String>) -> Result<String, ErrorMessage> {
    let password = password.into();

    if password.is_empty() {
        return Err(ErrorMessage::EmptyPassword);
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }
    let salt = SaltString::generate(&mut OsRng);

    //Argon 2 wigth default params
    let hashed_password = Argon2::default();

    //HashPassword
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|_| ErrorMessage::HashingError)?
        .to_string();
    Ok(hashed_password)
}

pub fn compare(password: &str, hashed_password: &str) -> Result<bool, ErrorMessage> {
    if password.is_empty() {
        return Err(ErrorMessage::EmptyPassword);
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }
    let parsed_hash =
        PasswordHash::new(hashed_password).map_err(|_| ErrorMessage::InvalidHashingFormat)?;
    let password_matches = Argon2
        .default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_or(false, |_| true);

    Ok(password_matches)
}
