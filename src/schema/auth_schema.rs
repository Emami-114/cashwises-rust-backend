use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::user_model::UserModel;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(
    length(min = 1, message = "Email is required"),
    email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
    #[validate(
    length(min = 1, message = "Please confirm your password"),
    must_match(other = "password", message = "Passwords do not match")
    )]
    #[serde(rename = "password_confirm")]
    pub password_confirm: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct VerificationDto {
    #[validate(
    length(min = 1, message = "Email is required"),
    email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(length(min = 4, message = "VerificationCode is required"))]
    pub code: Option<String>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    #[validate(
    length(min = 1, message = "Email is required"),
    email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    // #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    // #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &UserModel) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            email: user.email.to_owned(),
            name: user.name.to_owned(),
            photo: user.photo.to_owned(),
            role: user.role.to_str().to_string(),
            verified: user.verified,
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }

    pub fn filter_users(users: &[UserModel]) -> Vec<FilterUserDto> {
        users.iter().map(FilterUserDto::filter_user).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub status: String,
    pub users: Vec<FilterUserDto>,
    pub results: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}
