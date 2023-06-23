use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ControllerError {
    pub message: &'static str,
}
