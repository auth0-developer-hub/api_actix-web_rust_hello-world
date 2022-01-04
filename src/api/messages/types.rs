use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub api: String,
    pub branch: String,
    pub text: String,
}
