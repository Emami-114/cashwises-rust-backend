use crate::handlers::deal_handler::deals_scope;
use crate::handlers::image_handler::{get_image, upload_image_handler};
use actix_web::web;
use sqlx::{Pool, Postgres};

use crate::handlers::category_handler::category_scope;
use crate::handlers::{auth_handler::auth_scope, users_handler::users_scope};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(upload_image_handler)
        .service(get_image)
        .service(deals_scope())
        .service(auth_scope())
        .service(users_scope())
        .service(category_scope());
    conf.service(scope);
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
