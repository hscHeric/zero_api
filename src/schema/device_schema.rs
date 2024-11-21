use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateDeviceSchema {
    pub name: String,                // Nome do dispositivo
    pub description: Option<String>, // Descrição do dispositivo (opcional)
}
