use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct QuizMetadata {
    pub title: String,
    pub id: i64,
    pub created_at: String,
}