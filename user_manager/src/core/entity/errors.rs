use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DefaultError {
    pub message: String,
}

impl DefaultError {
    pub fn new(message: impl std::fmt::Display) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
