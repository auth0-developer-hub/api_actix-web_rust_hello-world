use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub message: &'static str,
}
