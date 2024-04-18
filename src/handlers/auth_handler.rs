use actix_web::{
    cookie::time::Duration as ActixWebDuration, cookie::Cookie, web, HttpResponse, Responder, Scope,
};
use actix_web::http::StatusCode;
use actix_web::web::Json;
use serde_json::json;
use validator::Validate;

use crate::{
    db::db_user_ext::UserExt,
    errors::{auth_errors::ErrorMessage, http_error::HttpError},
    extractors::auth::RequireAuth,
    schema::auth_schema::{
        FilterUserDto, LoginUserDto, RegisterUserDto, UserData, UserResponseDto,
    },
    utils::{password, token},
    AppState,
};
use crate::errors::auth_errors::ErrorResponse;
use crate::handlers::email_handler::EmailModel;
use crate::handlers::generate_random_string::generate_random_string;
use crate::handlers::users_handler::get_users;
use crate::schema::auth_schema::VerificationDto;

pub fn auth_scope() -> Scope {
    web::scope("/auth")
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        .route("/logout", web::post().to(logout).wrap(RequireAuth))
        .route("/verification", web::post().to(patch_verification_code))
}

pub async fn patch_verification_code(
    app_state: web::Data<AppState>,
    body: web::Json<VerificationDto>,
) -> Result<HttpResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;
    let result = app_state.db_client.set_verification_code(Some(&body.email.clone()), body.code.clone(),String::new()).await;
    match result {
        Ok(user) => {
            if let Some(user) = user {
                Ok(HttpResponse::Ok().finish())
            } else {
                Err(HttpError::bad_request("Failed Verification"))
            }
        },
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn register(
    app_state: web::Data<AppState>,
    body: web::Json<RegisterUserDto>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let hashed_password =
        password::hash(&body.password).map_err(|e| HttpError::server_error(e.to_string()))?;

    let result = app_state
        .db_client
        .save_customer(&body.name, &body.email, &hashed_password)
        .await;

    match result {
        Ok(user) => {
            let verification_code = generate_random_string(4);
            let email_instance = EmailModel::new(user.clone(), verification_code.clone(), app_state.email_config.clone());
            if let Err(err) = email_instance.send_verification_code().await {
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "Something bad happended while sending the verification code".to_string(),
                };
            }
          if let Err(err ) = app_state.db_client.set_verification_code(Some(&user.email.clone()), None, verification_code).await {
              ErrorResponse {
                  status: "fail".to_string(),
                  message: "Something bad happended while sending the verification code".to_string(),
              };
          }
            Ok(HttpResponse::Created().json(UserResponseDto {
                status: "success".to_string(),
                data: UserData {
                    user: FilterUserDto::filter_user(&user),
                },
            }))
        }
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_voilation(
                    ErrorMessage::EmailExist,
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn login(
    app_state: web::Data<AppState>,
    body: web::Json<LoginUserDto>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .get_user(None, None, Some(&body.email))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = result.ok_or(HttpError::unauthorized(ErrorMessage::WrongCredentials))?;

    let password_matches = password::compare(&body.password, &user.password)
        .map_err(|_| HttpError::unauthorized(ErrorMessage::WrongCredentials))?;

    if password_matches {
        let token = token::create_token(
            &user.id.to_string(),
            &app_state.env.jwt_secret.as_bytes(),
            app_state.env.jwt_maxage,
        )
            .map_err(|e| HttpError::server_error(e.to_string()))?;
        let cookie = Cookie::build("token", token.to_owned())
            .path("/")
            .max_age(ActixWebDuration::new(60 * &app_state.env.jwt_maxage, 0))
            .http_only(true)
            .finish();

        Ok(HttpResponse::Ok().cookie(cookie).json(json!({
            "token":token
        })))
    } else {
        Err(HttpError::unauthorized(ErrorMessage::WrongCredentials))
    }
}

pub async fn logout() -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();
    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}
