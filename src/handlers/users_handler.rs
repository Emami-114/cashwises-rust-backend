use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Scope};
use serde_json::json;
use validator::Validate;

use crate::{
    db::db_user_ext::UserExt,
    errors::http_error::HttpError,
    extractors::auth_middleware::{RequireAuth, RequireOnlyAdmin},
    models::user_model::UserModel,
    schema::auth_schema::{
        FilterUserDto, RequestQueryDto, UserListResponseDto,
    },
    AppState,
};

pub fn users_scope() -> Scope {
    web::scope("/users")
        .route("", web::get().to(get_users).wrap(RequireOnlyAdmin))
        .route("/me", web::get().to(get_me).wrap(RequireAuth))
}

async fn get_me(req: HttpRequest) -> Result<HttpResponse, HttpError> {
    match req.extensions().get::<UserModel>() {
        Some(user) => {
            let filtered_user = FilterUserDto::filter_user(user);

            // let response_data = UserResponseDto {
            //     status: "success".to_string(),
            //     data: UserData {
            //         user: filtered_user,
            //     },
            // };

            Ok(HttpResponse::Ok().json(json!(filtered_user)))
        }
        None => Err(HttpError::server_error("User not found")),
    }
}

pub async fn get_users(
    query: web::Query<RequestQueryDto>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDto = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let users = app_state
        .db_client
        .get_users(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(UserListResponseDto {
        status: "success".to_string(),
        users: FilterUserDto::filter_users(&users),
        results: users.len(),
    }))
}
