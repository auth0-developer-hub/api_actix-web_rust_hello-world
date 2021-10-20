use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    // TODO: Change type to String (separate PR)
    pub message: &'static str,
}
