use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub metadata: Metadata,
    pub text: String,
}

#[derive(Serialize)]
pub struct Metadata {
    pub api: String,
    pub branch: String,
}
