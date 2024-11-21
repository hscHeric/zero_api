use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct SensorReading {
    pub id: i32,                     // ID único para cada leitura de sensor
    pub device_id: Uuid,             // ID do sensor relacionado
    pub reading_time: DateTime<Utc>, // Data e hora da leitura
    pub amperes: f64,                // Número de amperes
    pub voltage: f64,                // Voltagem
}
