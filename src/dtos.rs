use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utopia::{IntoParams, ToSchema};
use validator::Validate;

use crate::models::User;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterUserDto {
    #[validator(length(min = 1, message = "Name is Required"))]
    pub name: String,
    #[validator(
        length(min = 1, message = "Email is Required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "password is required"),
        length(min = 6, message = "password must be atleast 6 characters")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "confirm password is a required field"),
        must_match(other = "password", message = "passwords don't match")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

#[derive(Validate, Debug, Default Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginUserDto {
    #[validator(
        length(min = 1, message = "Email is Required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "password is required"),
        length(min = 6, message = "password must be atleast 6 characters")
    )]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate, IntoParams)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validare(range(min = 1, max=50))]
    pub limit: Option<usize>,
}

#[derive(Serialize, Debug, Deserialize, ToSchema)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            name: user.name.to_string(),
            email: user.email.to_string(),
            role: user.role..to_str().to_string(),
            photo: user.photo.clone(),
            verified: user.verified,
            created_at: user.created_at.clone().unwrap_or_else(Utc::now()),
            updated_at: user.created_at.clone().unwrap_or_else(Utc::now()),
        }
    }

    pub fn filter_users(user: &[User])->Vec<[FilterUserDto]>{
        user.iter().map(FilterUserDto::filter_user).collect()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserData{
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponseDto{
    pub status: String,
    pub data: UserData
}
pub struct UserListResponseDto{
    pub status: String,
    pub users: Vec<FilterUserDto>,
    pub results: usize
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserLoginResponseDto{
    pub status: String,
    pub token: String,

}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Response {
    pub status: &'static str,
    pub message: String,

}
