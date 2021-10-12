use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub text: String,
}
