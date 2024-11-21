use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    db::app_state::AppState,
    models::{device::Device, sensor_reading::SensorReading},
    schema::sensor_reading_schema::CreateSensorReadingSchema,
};

async fn insert_reading(
    pool: &Pool<Postgres>,
    reading: &CreateSensorReadingSchema,
) -> Result<SensorReading, sqlx::Error> {
    let query = r#"
        INSERT INTO sensor_readings (device_id, reading_time, amperes, voltage)
        VALUES ($1, $2, $3, $4)
        RETURNING id, device_id, reading_time, amperes, voltage
    "#;

    let row: (i32, Uuid, DateTime<Utc>, f64, f64) = sqlx::query_as(query)
        .bind(&reading.device_id)
        .bind(&reading.reading_time)
        .bind(&reading.amperes)
        .bind(&reading.voltage)
        .fetch_one(pool)
        .await?;

    Ok(SensorReading {
        id: row.0,
        device_id: row.1,
        reading_time: row.2,
        amperes: row.3,
        voltage: row.4,
    })
}

// Rota para criação de leitura
#[post("/devices/{device_id}/readings")]
pub async fn create_reading(
    data: Data<AppState>,
    path: web::Path<(Uuid,)>,
    body: Json<CreateSensorReadingSchema>,
) -> impl Responder {
    let device_id = path.0;

    // Check if device exists (optional)
    let device_exists = Device::exists(&data.db, device_id).await.unwrap_or(false);

    if !device_exists {
        return HttpResponse::NotFound().json(serde_json::json!({
            "status": "error",
            "message": format!("Device with ID {} not found", device_id)
        }));
    }

    match insert_reading(&data.db, &body).await {
        Ok(reading) => {
            let reading_response = serde_json::json!({
                "status": "success",
                "reading": reading
            });

            HttpResponse::Created().json(reading_response)
        }
        Err(e) => {
            eprintln!("Error inserting reading: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", e)
            }))
        }
    }
}
