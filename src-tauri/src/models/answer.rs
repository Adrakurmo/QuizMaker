use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub text: String,
    pub is_correct: bool,
}