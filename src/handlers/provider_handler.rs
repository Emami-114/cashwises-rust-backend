use actix_web::{HttpResponse, Responder, Scope, web};
use serde_json::json;
use crate::AppState;
use crate::extractors::auth_middleware::{RequireOnlyAdmin, RequireOnlyCreatorAndAdmin};
use crate::models::provider_model::ProviderModel;
use crate::schema::provider_schema::{CreateProvider, ProviderFilterOptions};

pub fn provider_scope() -> Scope {
    web::scope("/providers")
        .route("", web::post().to(create_provider_handler).wrap(RequireOnlyCreatorAndAdmin))
        .route("", web::get().to(provider_list_handler).wrap(RequireOnlyCreatorAndAdmin))
        .route("/{id}", web::delete().to(delete_provider_handler).wrap(RequireOnlyAdmin))
}

async fn create_provider_handler(
    body: web::Json<CreateProvider>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        ProviderModel,
        "INSERT INTO providers (title,logo,url) VALUES ($1,$2,$3) RETURNING *",
        body.title,
        body.logo,
        body.url
    )
        .fetch_one(&data.db_client.pool).await;
    return match query_result {
        Ok(provider) => {
            HttpResponse::Ok().json(json!(provider))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::Conflict()
                    .json(serde_json::json!({"status": "fail","message": "Provider with that title already exists"}));
            }
            HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message": format!("{:?}",e)
            }))
        }
    };
}

async fn provider_list_handler(
    opts: web::Query<ProviderFilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = if let Some(query_text) = &opts.query {
        sqlx::query_as!(
            ProviderModel,
             "SELECT * FROM providers WHERE CASE
            WHEN $1 <> '' THEN LOWER(title) LIKE '%' || LOWER($1) || '%'
            ELSE true
            END ORDER BY title LIMIT $2 OFFSET $3",
            query_text,
            limit as i32,
            offset as i32
        ).fetch_all(&data.db_client.pool).await
    } else {
        sqlx::query_as!(
            ProviderModel,
            "SELECT * FROM providers ORDER BY title LIMIT $1 OFFSET $2",
            limit as i32,
            offset as i32
        ).fetch_all(&data.db_client.pool).await
    };

    HttpResponse::Ok().json(json!(query_result.unwrap()))
}

async fn delete_provider_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let provider_id = path.into_inner();
    let rows_affected = sqlx::query_as!(
        ProviderModel,
        "DELETE FROM providers WHERE id = $1",
        provider_id
    )
        .execute(&data.db_client.pool)
        .await
        .unwrap()
        .rows_affected();
    if rows_affected == 0 {
        let message = format!("Provider with ID: {} note found", provider_id);
        return HttpResponse::NotFound().json(json!({"status":"fail","message":message}));
    }
    HttpResponse::NoContent().finish()
}
