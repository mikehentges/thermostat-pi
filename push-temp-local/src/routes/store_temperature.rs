//use actix_web::http::header::ContentType;
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::types::chrono::DateTime;
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use std::str::FromStr;
use temp_data::TempData;

#[tracing::instrument(name = "storing the temperature")]
pub async fn store_temperature(
    body: web::Json<TempData>,
    pool: web::Data<PgPool>,
    request: HttpRequest,
) -> HttpResponse {
    tracing::info!("setting temperature of: {:?}", body);

    let result = store_result(&pool, body).await;
    match result {
        Err(e) => tracing::error!("error running store_result: {:?}", e),
        Ok(_) => tracing::info!("store_result returned OK"),
    }

    HttpResponse::Ok().finish()
}

#[tracing::instrument(name = "storing the temperature2")]
async fn store_result(
    pool: &PgPool,
    body: web::Json<TempData>,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("running store_result");

    let input_dt = DateTime::parse_from_rfc3339(body.record_date.as_str())?;
    let date = input_dt.date_naive();

    let _result = sqlx::query!(
        r#"
    insert into
    shop_thermostat (
        record_day,
        record_date,
        temperature,
        thermostat_value,
        thermostat_on
    )
values
    ( $1, $2, $3, $4, $5 )
    "#,
        date,
        input_dt,
        BigDecimal::from_str(format!("{}", body.temperature).as_str())?,
        body.thermostat_value as i32,
        body.thermostat_on
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        return e;
    });
    Ok(())
}
