use crate::handlers::config::{DBClient, DBConfig};
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use utoipa::Modify;
use utoipa::openapi::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

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
                    .build()
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

    dotenv().ok();
    env_logger::init();

    let db_config = DBConfig::init();
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
        Ok(_) => println!("Migrations executed successfully."),
        Err(e) => eprintln!("Error executing migrations: {}", e),
    };

    let db_client = DBClient::new(pool);
    let app_state: AppState = AppState {
        env: db_config.clone(),
        db_client,
    };
    println!(
        "🚀 Server running on http://localhost:{}",
        db_config.port
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8080")
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
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0", db_config.port))?
        .run()
        .await?;
    Ok(())
}
