use crate::{
    models::deal_model::DealModel,
    schema::deal_schema::{CreateDealSchema, UpdateDealSchema},
    AppState,
};
use actix_web::{web, HttpResponse, Responder, Scope};
use chrono::prelude::*;
use serde_json::json;
use crate::extractors::auth::RequireAuth;
use crate::schema::response_schema::FilterOptions;


pub fn deals_scope() -> Scope {
    web::scope("/deals")
        .route("", web::post().to(create_deal_handler))
        .route("", web::get().to(deal_list_handler))
        .route("/{id}", web::get().to(get_deal_handler))
        .route("/{id}", web::patch().to(edit_deal_handler))
        .route("/{id}", web::delete().to(delete_deal_handler))
}

async fn create_deal_handler(
    body: web::Json<CreateDealSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        DealModel,
        "INSERT INTO deals (title,description,category,is_free,price,offer_price,published,expiration_date,provider,provider_url,thumbnail,images,user_id,video_url) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14) RETURNING *",
        body.title,
        body.description,
        body.category.as_deref(),
        body.is_free,
        body.price,
        body.offer_price,
        body.published,
        body.expiration_date,
        body.provider,
        body.provider_url,
        body.thumbnail,
        body.images.as_deref(),
        body.user_id,
        body.video_url,
    )
        .fetch_one(&data.db_client.pool)
        .await;
    match query_result {
        Ok(deal) => {
            let deal_response = json!({"status":"success","data": json!({
                "deal": deal
            })});
            return HttpResponse::Ok().json(deal_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "fail","message": "Note with that title already exists"}));
            }
            return HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message": format!("{:?}",e)
            }));
        }
    }
}

async fn deal_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_count = sqlx::query_scalar!("SELECT COUNT(*) FROM deals")
        .fetch_one(&data.db_client.pool)
        .await
        .unwrap()
        .unwrap_or(0);
    let pages_count = query_count / limit as i64;

    let query_result = if let Some(query_text) = &opts.query {
        sqlx::query_as!(
            DealModel,
            "SELECT * FROM deals WHERE CASE
            WHEN $1 <> '' THEN title LIKE '%' || $1 || '%'
            ELSE true
            END ORDER BY updated_at LIMIT $2 OFFSET $3",
            query_text,
            limit as i32,
            offset as i32
        ).fetch_all(&data.db_client.pool).await
    } else {
        sqlx::query_as!(
            DealModel,
            "SELECT * FROM deals ORDER BY updated_at LIMIT $1 OFFSET $2",
            limit as i32,
            offset as i32
        ).fetch_all(&data.db_client.pool).await
    };

    if query_result.is_err() {
        let message = "Something bad happened while fetching all deal items";
        return HttpResponse::InternalServerError().json(json!({
            "status":"error",
            "message":message
        }));
    }
    let deals = query_result.unwrap();
    let json_response = serde_json::json!({
        "status":"success",
        "results":deals.len(),
        "pages":pages_count,
        "deals":deals
    });

    HttpResponse::Ok().json(json_response)
}

async fn get_deal_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let deal_id = path.into_inner();
    let query_result = sqlx::query_as!(DealModel, "SELECT * FROM deals WHERE id = $1", deal_id)
        .fetch_one(&data.db_client.pool)
        .await;

    match query_result {
        Ok(deal) => {
            let deal_response = json!({
                "status":"success",
                "data": json!({"deal": deal})
            });
            return HttpResponse::Ok().json(deal_response);
        }
        Err(_) => {
            let message = format!("Note with ID: {} not found", deal_id);
            return HttpResponse::NotFound().json(json!({
                "status":"fail",
                "message":message
            }));
        }
    }
}

async fn edit_deal_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateDealSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let deal_id = path.into_inner();
    let query_result = sqlx::query_as!(DealModel, "SELECT * FROM deals WHERE id = $1", deal_id)
        .fetch_one(&data.db_client.pool)
        .await;
    if query_result.is_err() {
        let message = format!("Note with ID: {} not found", deal_id);
        return HttpResponse::NotFound().json(json!({
            "status":"fail",
            "message":message
        }));
    }
    let now = Utc::now();

    let query_result = sqlx::query_as!(
        DealModel,
        "UPDATE deals SET title = COALESCE($1, title), description = COALESCE($2, description), category = COALESCE($3, category), is_free = COALESCE($4, is_free), price = COALESCE($5, price), offer_price = COALESCE($6, offer_price), published = COALESCE($7, published), expiration_date = COALESCE($8, expiration_date), provider = COALESCE($9, provider), provider_url = COALESCE($10, provider_url), thumbnail = COALESCE($11, thumbnail),images = COALESCE($12, images),user_id = COALESCE($13, user_id),video_url = COALESCE($14, video_url), updated_at = $15 WHERE id = $16 RETURNING *",
         body.title,
        body.description,
        body.category.as_deref(),
        body.is_free,
        body.price,
        body.offer_price,
        body.published,
        body.expiration_date,
        body.provider,
        body.provider_url,
        body.thumbnail,
        body.images.as_deref(),
        body.user_id,
        body.video_url,
        now,
        deal_id,
    ).fetch_one(&data.db_client.pool)
        .await;

    match query_result {
        Ok(deal) => {
            let deal_response = json!({
                "status":"success",
                "data": serde_json::json!({"data":deal})
            });
            return HttpResponse::Ok().json(deal_response);
        }

        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status":"error","message":message}));
        }
    }
}

async fn delete_deal_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let deal_id = path.into_inner();
    let rows_affected = sqlx::query_as!(DealModel, "DELETE FROM deals WHERE id = $1", deal_id)
        .execute(&data.db_client.pool)
        .await
        .unwrap()
        .rows_affected();
    if rows_affected == 0 {
        let message = format!("Note with ID: {} note found", deal_id);
        return HttpResponse::NotFound().json(json!({"status":"fail","message":message}));
    }
    HttpResponse::NoContent().finish()
}



