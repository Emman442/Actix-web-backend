use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utopia::{IntoParams, ToSchema};
use validator::Validate;


use crate::models::User;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterUserDto{
    #[validator(length(min=1, message="Name is Required"))]
    pub name: String,
    #[validator(length(min=1,message="Email is Required"), email(message="Email is invalid"))]
    pub email: String,
    #[validate(length)]
    pub password: String,


}