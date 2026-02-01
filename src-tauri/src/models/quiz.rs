use serde::{Deserialize, Serialize};
use crate::models::question::Question;

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    pub id: i64,
    pub title: String,
    pub created_at: String,
    pub questions: Vec<Question>,
}