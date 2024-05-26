use actix_web::{HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;
use crate::AppState;
use crate::models::user_marked_deals::UserMarkedDeals;

pub async fn mark_deal_for_user(
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
        Err(e) => {
            HttpResponse::InternalServerError()
        }
    };
}


pub async fn delete_mark_deal_user(
    path: actix_web::web::Path<Uuid>,
    data: actix_web::web::Data<AppState>,
) -> impl Responder {
    let deal_id = path.into_inner();
    let rows_affected = sqlx::query_as!(
        UserMarkedDeals,
        "DELETE FROM user_marked_deals WHERE deal_id = $1",
        deal_id
    ).execute(&data.db_client.pool).await.unwrap().rows_affected();
    if rows_affected == 0 {
        let message = format!("Deal marked with ID: {} not found", deal_id);
        return HttpResponse::NotFound().json(json!(message));
    }
    HttpResponse::NoContent().finish()
}