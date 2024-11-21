use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    db::app_state::AppState, models::device::Device, schema::device_schema::CreateDeviceSchema,
};

async fn insert_device(
    pool: &Pool<Postgres>,
    device: &CreateDeviceSchema,
) -> Result<Device, sqlx::Error> {
    let query = r#"
        INSERT INTO devices (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description, created_at, updated_at
    "#;

    let row: (Uuid, String, Option<String>, DateTime<Utc>, DateTime<Utc>) = sqlx::query_as(query)
        .bind(&device.name)
        .bind(&device.description)
        .fetch_one(pool)
        .await?;

    Ok(Device {
        id: row.0,
        name: row.1,
        description: row.2,
        created_at: Some(row.3),
        updated_at: Some(row.4),
    })
}

// Rota de criação do dispositivo
#[post("/device")]
pub async fn create_device(
    body: Json<CreateDeviceSchema>,
    data: Data<AppState>, // Usando o AppState para acessar o db
) -> impl Responder {
    match insert_device(&data.db, &body).await {
        Ok(device) => {
            let device_response = serde_json::json!({
                "status": "success",
                "device": device
            });
            HttpResponse::Created().json(device_response) // Retorna o dispositivo recém-criado
        }
        Err(e) => {
            eprintln!("Error inserting device: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", e)
            }))
        }
    }
}
