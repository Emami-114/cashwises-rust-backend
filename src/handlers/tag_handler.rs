use actix_web::{web, HttpResponse, Responder, Scope};
use serde_json::json;
use crate::{AppState};
use crate::extractors::auth_middleware::{RequireOnlyAdmin, RequireOnlyCreatorAndAdmin};
use crate::models::tag_model::{TagModel, CreateTagSchema};
use crate::schema::response_schema::FilterOptions;

pub fn tags_scope() -> Scope {
    web::scope("/tags")
        .route("", web::post().to(create_tag_handler).wrap(RequireOnlyCreatorAndAdmin))
        .route("", web::get().to(tag_list_handler))
        .route("/{id}", web::get().to(get_tag_handler))
        .route("/{id}", web::delete().to(delete_tag_handler).wrap(RequireOnlyAdmin))
}

async fn create_tag_handler(
    body: web::Json<CreateTagSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        TagModel,
        "INSERT INTO tags (title) VALUES ($1) RETURNING *",
        body.title.to_string(),
    )
        .fetch_one(&data.db_client.pool)
        .await;
    return match query_result {
        Ok(tag) => {
            HttpResponse::Ok().json(json!(tag))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "fail","message": "Note with that title already exists"}));
            }
            HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message": format!("{:?}",e)
            }))
        }
    };
}

async fn tag_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = if let Some(query_text) = &opts.query {
        sqlx::query_as!(
            TagModel,
            "SELECT * FROM tags WHERE CASE
            WHEN $1 <> '' THEN LOWER(title) LIKE '%' || LOWER($1) || '%'
            ELSE true
            END ORDER BY created_at LIMIT $2 OFFSET $3",
            query_text,
            limit as i32,
            offset as i32
        )
            .fetch_all(&data.db_client.pool)
            .await
    } else {
        sqlx::query_as!(
            TagModel,
            "SELECT * FROM tags ORDER BY created_at LIMIT $1 OFFSET $2",
            limit as i32,
            offset as i32
        )
            .fetch_all(&data.db_client.pool)
            .await
    };
    return match query_result {
        Ok(tags) => {
            let json_response = serde_json::json!(tags);
            HttpResponse::Ok().json(json_response)
        }
        Err(_) => {
            let message = "Something bad happened while fetching all tags items";
            return HttpResponse::InternalServerError().json(json!({
            "status":"error",
            "message":message
        }));
        }
    };
}

async fn get_tag_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let tag_id = path.into_inner();
    let query_result = sqlx::query_as!(
        TagModel,
        "SELECT * FROM tags WHERE id = $1",
        tag_id
    )
        .fetch_one(&data.db_client.pool)
        .await;
    return match query_result {
        Ok(tag) => {
            HttpResponse::Ok().json(json!(tag))
        }
        Err(e) => {
            let message = format!("tag with ID: {} not found err: {}", tag_id, e);
            HttpResponse::NotFound().json(json!({
                "status":"fail",
                "message":message
            }))
        }
    };
}

async fn delete_tag_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let tag_id = path.into_inner();
    let rows_affected = sqlx::query_as!(
        TagModel,
        "DELETE FROM tags WHERE id = $1",
        tag_id
    )
        .execute(&data.db_client.pool)
        .await
        .unwrap()
        .rows_affected();
    if rows_affected == 0 {
        let message = format!("Tag with ID: {} note found", tag_id);
        return HttpResponse::NotFound().json(json!({"status":"fail","message":message}));
    }
    HttpResponse::NoContent().finish()
}