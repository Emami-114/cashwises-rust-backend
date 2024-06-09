use actix_web::{HttpResponse, Responder, web};
use serde_json::json;
use uuid::Uuid;
use crate::AppState;
use crate::models::user_marked_deals::{UserMarkedDeals};

pub async fn post_mark_deal_for_user(
    body: actix_web::web::Json<UserMarkedDeals>,
    data: actix_web::web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        UserMarkedDeals,
        r#"INSERT INTO user_marked_deals (user_id,deal_id) VALUES ($1,$2)"#,
        body.user_id,
        body.deal_id
    )
        .execute(&data.db_client.pool).await;
    return match query_result {
        Ok(_) => {
            HttpResponse::Ok()
        }
        Err(_) => {
            HttpResponse::InternalServerError()
        }
    };
}

pub async fn get_list_mark_deal_for_user(
    path: actix_web::web::Path<Uuid>,
    data: actix_web::web::Data<AppState>,
) -> impl Responder {
    let user_id = path.into_inner();
    let query_results = sqlx::query_as!(
        UserMarkedDeals,
        "SELECT * FROM user_marked_deals WHERE user_id = $1",
        user_id
    )
        .fetch_all(&data.db_client.pool).await;
    match query_results {
        Ok(deals) => {
            HttpResponse::Ok().json(deals)
        }
        Err(_) => {
            HttpResponse::BadRequest().json(())
        }
    }

}

pub async fn delete_mark_deal_user(
    path: web::Path<(Uuid, Uuid)>,
    data: actix_web::web::Data<AppState>,
) -> impl Responder {
    let (user_id, deal_id) = path.into_inner();

    let rows_affected = sqlx::query_as!(
        UserMarkedDeals,
        "DELETE FROM user_marked_deals WHERE user_id = $1 AND deal_id = $2",
        user_id,
        deal_id
    ).execute(&data.db_client.pool).await.unwrap().rows_affected();
    if rows_affected == 0 {
        let message = "Deal marked with ID: not found".to_string();
        return HttpResponse::NotFound().json(json!(message));
    }
    HttpResponse::NoContent().finish()
}