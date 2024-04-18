use crate::handlers::config::{DBClient, DBConfig, EmailConfig};
use crate::handlers::generate_random_string::generate_random_string;
use crate::models::user_model::{UserModel, UserRole};
use actix_cors::Cors;
use actix_web::{
    get, http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use rand::Rng;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::path::Path;
use tokio::fs;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::openapi::OpenApi;
use utoipa::Modify;

mod db;
mod errors;
mod extractors;
mod handlers;
mod models;
mod schema;
mod utils;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: DBConfig,
    pub db_client: DBClient,
    pub email_config: EmailConfig,
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    openssl_probe::init_ssl_cert_env_vars();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    if !Path::new("./uploads").exists() {
        fs::create_dir("./uploads").await?;
    }
    dotenv().ok();
    env_logger::init();

    let db_config = DBConfig::init();
    let email_config = EmailConfig::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_config.database_url)
        .await
    {
        Ok(pool) => {
            println!("🤘 Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Migrations executed successfully.✌️"),
        Err(e) => eprintln!("Error executing migrations: {}", e),
    };

    let db_client = DBClient::new(pool);
    let app_state: AppState = AppState {
        env: db_config.clone(),
        db_client,
        email_config,
    };
    println!("🚀 Server running on http://localhost:{}", db_config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(handlers::config::config)
            .service(health_checker_handler)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", db_config.port))?
    .run()
    .await?;
    Ok(())
}
#[get("/")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "BUILD SIMPLE CRUD API with RUST";
    HttpResponse::Ok().json(json!({
        "status":"success",
        "message": MESSAGE
    }))
}
