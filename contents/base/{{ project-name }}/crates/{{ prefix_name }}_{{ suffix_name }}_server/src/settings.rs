use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub management_port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: {{ service_port }},
            management_port: {{ management_port }},
        }
    }
}
