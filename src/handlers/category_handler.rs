use crate::extractors::auth::RequireAuth;
use crate::schema::category_shema::UpdateCategorySchema;
use crate::{
    models::category_model::CategoryModel, schema::category_shema::CreateCategorySchema,
    schema::response_schema::FilterOptions, AppState,
};
use actix_web::{web, HttpResponse, Responder, Scope};
use chrono::Utc;
use serde_json::json;

pub fn category_scope() -> Scope {
    web::scope("")
        .route(
            "/category",
            web::post().to(create_category_handler).wrap(RequireAuth),
        )
        .route(
            "/categories",
            web::get().to(category_list_handler).wrap(RequireAuth),
        )
        .route(
            "/category/{id}",
            web::get().to(get_category_handler).wrap(RequireAuth),
        )
        .route(
            "/category/{id}",
            web::patch().to(patch_category_handler).wrap(RequireAuth),
        )
        .route(
            "/category/{id}",
            web::delete().to(delete_category_handler).wrap(RequireAuth),
        )
}

async fn create_category_handler(
    body: web::Json<CreateCategorySchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        CategoryModel,
        "INSERT INTO categories (title,thumbnail,user_id,published,status,main_id) VALUES ($1,$2,$3,$4,$5,$6) RETURNING *",
        body.title,
        body.thumbnail,
        body.user_id,
        body.published,
        body.status,
        body.main_id
    )
        .fetch_one(&data.db_client.pool)
        .await;
    return match query_result {
        Ok(category) => {
            let category_response = json!({
                "status":"success",
                "data": json!({"category":category})
            });
            HttpResponse::Ok().json(category_response)
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::Conflict()
                    .json(serde_json::json!({"status": "fail","message": "Category with that title already exists"}));
            }
            HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message": format!("{:?}",e)
            }))
        }
    };
}

async fn category_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_count = sqlx::query_scalar!("SELECT COUNT(*) FROM categories")
        .fetch_one(&data.db_client.pool)
        .await
        .unwrap()
        .unwrap_or(0);
    let pages_count = query_count / limit as i64;
    let query_result = sqlx::query_as!(
        CategoryModel,
        "SELECT * FROM categories ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db_client.pool)
    .await;
    if query_result.is_err() {
        let message = "Something bad happened while fetching all deal items";
        return HttpResponse::InternalServerError().json(json!({
            "status":"error",
            "message":message
        }));
    }
    let categories = query_result.unwrap();
    let pages_count = if pages_count < 1 { 1 } else { pages_count };
    let json_response = json!({
        "status": "success",
        "results": categories.len(),
        "pages":pages_count,
        "categories": categories
    });
    HttpResponse::Ok().json(json_response)
}

async fn get_category_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let category_id = path.into_inner();
    let query_result = sqlx::query_as!(
        CategoryModel,
        "SELECT * FROM categories WHERE id = $1",
        category_id
    )
    .fetch_one(&data.db_client.pool)
    .await;

    return match query_result {
        Ok(category) => {
            HttpResponse::Ok().json(category)
        }
        Err(_) => {
            let message = format!("category with ID: {} not found", category_id);
            HttpResponse::NotFound().json(json!({
                "status":"fail",
                "message":message
            }))
        }
    }
}

async fn patch_category_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateCategorySchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let category_id = path.into_inner();
    let query_result = sqlx::query_as!(
        CategoryModel,
        "SELECT * FROM categories WHERE id = $1",
        category_id
    )
    .fetch_one(&data.db_client.pool)
    .await;

    if query_result.is_err() {
        let message = format!("Category with ID: {} not found", category_id);
        return HttpResponse::NotFound().json(json!({
            "status":"fail",
            "message":message
        }));
    }

    let now_date_time = Utc::now();
    let query_result = sqlx::query_as!(
        CategoryModel,
        "UPDATE categories SET title = COALESCE($1, title), thumbnail = COALESCE($2, thumbnail),user_id = COALESCE($3, user_id), published = COALESCE($4, published),status = COALESCE($5, status), main_id = COALESCE($6, main_id),updated_at = $7 WHERE id = $8 RETURNING *",
        body.title,
        body.thumbnail,
        body.user_id,
        body.published,
        body.status,
        body.main_id,
        now_date_time,
        category_id
    ).fetch_one(&data.db_client.pool).await;

    match query_result {
        Ok(category) => {
            return HttpResponse::Ok().json(category);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status":"error","message":message}));
        }
    }
}

async fn delete_category_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let category_id = path.into_inner();
    let rows_affected = sqlx::query_as!(
        CategoryModel,
        "DELETE FROM categories WHERE id = $1",
        category_id
    )
    .execute(&data.db_client.pool)
    .await
    .unwrap()
    .rows_affected();
    if rows_affected == 0 {
        let message = format!("Category with ID: {} note found", category_id);
        return HttpResponse::NotFound().json(json!({"status":"fail","message":message}));
    }
    HttpResponse::NoContent().finish()
}
