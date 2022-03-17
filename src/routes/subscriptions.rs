use actix_web::{web, App, HttpResponse, HttpServer};
use chrono::Utc;
use sqlx::{PgConnection, PgPool};
use tracing::Instrument;
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form,pool),
    fields(
        subscriber_name = %form.name,
        subscriber_email = %form.email
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&pool, &form).await
    {
        Ok(_) => HttpResponse::Ok().finish(),

        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}
#[tracing::instrument(
    name = "Saveing new subscriber details in the databse ",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email , name , subscribed_at) VALUES ($1,$2,$3,$4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    ).execute(pool).await.map_err(|e| {
            tracing::error!("failed to execute query:{:?}", e);
            e
        })?;
    Ok(())
}
