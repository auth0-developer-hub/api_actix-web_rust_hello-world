use std::fmt::{Formatter, Result};
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub client_origin_url: String,
}

impl Default for Config {
    fn default() -> Self {
        envy::from_env::<Config>().expect("Provide missing environment variables for Config")
    }
}

#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    pub message: String,
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message)
    }
}