use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Device {
    pub(crate) async fn exists(db: &Pool<Postgres>, device_id: Uuid) -> Option<bool> {
        let query = sqlx::query_scalar!(
            "SELECT EXISTS (SELECT 1 FROM devices WHERE id = $1)",
            device_id
        );

        match query.fetch_one(db).await {
            Ok(result) => result,
            Err(err) => {
                eprintln!("Error checking device existence: {:?}", err);
                None
            }
        }
    }
}
