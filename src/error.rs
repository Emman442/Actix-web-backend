use crate::dtos::Response;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Error, format};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt {
        write(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    HashingError,
    InvalidHashingFormat,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExists,
    UserNoLongerExists,
    TokenNotProvided,
    PermissionDenied,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}
impl Into<String> for ErrorMessage {
    fn into(self) -> String {
        self.to_string()
    }
}
impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::ServerError => "Server Error. Please try again!".to_string(),
            ErrorMessage::EmailExists => {
                "Email Already Exists, Please use another email!".to_string()
            }
            ErrorMessage::WrongCredentials => "Email or Password is wrong".to_string(),
            ErrorMessage::InvalidToken => "Email is Invalid!, Please login again".to_string(),
            ErrorMessage::UserNoLongerExists => {
                "User belonging to this token does no longer exists".to_string()
            }
            ErrorMessage::EmptyPassword => "Password csnnot be empty".to_string(),
            ErrorMessage::HashingError => "Error While Hashing password".to_string(),
            ErrorMessage::InvalidHashingFormat => "Invalid Password Hashing format".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(max_length) => {
                format!("Password must not be more than {} characters", max_length)
            }
            ErrorMessage::TokenNotProvided => {
                "You are not logged in, please provide a token".to_string()
            }
            ErrorMessage::PermissionDenied => {
                "You are not authorized to perform this action".to_string()
            }
        }
    }
}

pub struct HttpError{
    message: String,
    status: u16
}

impl HttpError{
    pub fn new(message: impl Into<String>, status:u16)->Self{
        HttpError{
            message: message.into(),
            status
        }
    }

    pub fn server_error(message: impl Into<String>)->Self{
        HttpError{
            message: message.into(),
            status: 500
        }
    }

    pub fn bad_request(message: impl Into<String>)->Self{
        HttpError{
            message: message.into(),
            status: 500
        }
    }

    pub fn unique_constraint_violation(message: impl Into<String>)->Self{
        HttpError{
            message: message.into(),
            status: 500
        }
    }

    pub fn unauthorized(message: impl Into<String>)->Self{
        HttpError{
            message: message.into(),
            status: 500
        }
    }

    pub fn into_http_response(self)->HttpResponse{
        match self.status{
            400=>HttpResponse::BadRequest().json(Response{
                status: "fail",
                message: self.message.into()
            }),
            401=>HttpResponse::Unauthorized().json(Response{
                status: "fail",
                message: self.message.into()
            }),
            409=>HttpResponse::Conflict().json(Response{
                status: "fail",
                message: self.message.into()
            }),
            500=>HttpResponse::InternalServerError().json(Response{
                status: "fail",
                message: self.message.into()
            }),
            _=>{
                eprint!(
                   "Warning: Missing Pattern match. Converted status code {} for 500."
                );

                HttpResponse::InternalServerError().json(Response{
                    status: "error",
                    message: ErrorMessage::ServerError.into()
                })
            }

        }
    }
}

impl fmt::Display for HttpError{
    fn fmt(&self, f: &mut fmt:: Formatter<'_>)->fmt::Result{
        write!(f, "HttpError: message: {}, status: {}", self.message, self.status)
    }
}

impl std::error::Error for HttpError{}

impl ResponseError for HttpError{
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody>{
        let cloned = self.clone();
        cloned.into_http_response()
    }
}