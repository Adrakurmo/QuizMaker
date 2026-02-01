use crate::models::answer::Answer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub text: String,
    pub  answers: Vec<Answer>
}