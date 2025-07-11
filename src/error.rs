use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct GameError {
    message: String,
}

#[wasm_bindgen]
impl GameError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game Error: {}", self.message)
    }
}

impl std::error::Error for GameError {}

pub type Result<T> = std::result::Result<T, GameError>;
