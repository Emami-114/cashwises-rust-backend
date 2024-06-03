use crate::handlers::deal_handler::deals_scope;
use crate::handlers::image_handler::image_scope;
use actix_web::web;
use sqlx::{Pool, Postgres};
use crate::extractors::api_key_middleware::ApiKeyMiddleware;

use crate::handlers::category_handler::category_scope;
use crate::handlers::{auth_handler::auth_scope, users_handler::users_scope};
use crate::handlers::provider_handler::provider_scope;
use crate::handlers::tag_handler::tags_scope;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(image_scope())
        .service(provider_scope().wrap(ApiKeyMiddleware))
        .service(tags_scope().wrap(ApiKeyMiddleware))
        .service(deals_scope().wrap(ApiKeyMiddleware))
        .service(auth_scope().wrap(ApiKeyMiddleware))
        .service(users_scope().wrap(ApiKeyMiddleware))
        .service(category_scope().wrap(ApiKeyMiddleware));
    conf.service(scope);
}

#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_from: String,
    pub smtp_to: String,
}

impl EmailConfig {
    pub fn init() -> EmailConfig {
        let smtp_host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_port = std::env::var("SMTP_PORT").expect("SMTP_PORT must be set");
        let smtp_user = std::env::var("SMTP_USER").expect("SMTP_USER must be set");
        let smtp_pass = std::env::var("SMTP_PASS").expect("SMTP_PASS must be set");
        let smtp_from = std::env::var("SMTP_FROM").expect("SMTP_FROM must be set");
        let smtp_to = std::env::var("SMTP_TO").expect("SMTP_TO must be set");
        EmailConfig {
            smtp_host,
            smtp_port: smtp_port.parse::<u16>().unwrap(),
            smtp_user,
            smtp_pass,
            smtp_from,
            smtp_to,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DBClient {
    pub pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        DBClient { pool }
    }
}

#[derive(Debug, Clone)]
pub struct DBConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16,
}

impl DBConfig {
    pub fn init() -> DBConfig {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");

        DBConfig {
            database_url,
            jwt_secret,
            jwt_maxage: jwt_maxage.parse::<i64>().unwrap(),
            port: 8000,
        }
    }
}
