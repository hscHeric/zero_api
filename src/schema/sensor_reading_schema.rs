use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CreateSensorReadingSchema {
    pub device_id: Uuid,
    pub reading_time: DateTime<Utc>,
    pub amperes: f64,
    pub voltage: f64,
}
