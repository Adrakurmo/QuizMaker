use sqlx::SqlitePool;
use tauri::State;

use crate::{models::{question::Question, quizmetadata::QuizMetadata}, services::{db::{add_quiz_to_db, get_quizzes_metadata_db}, quiz_service::validate_quiz_data}};

#[tauri::command]
pub async fn add_quiz(quiz_name: String, questions: Vec<Question>, pool: State<'_, SqlitePool>) -> Result<(), String> {
    match validate_quiz_data(&quiz_name, questions) {
        Ok(obj) => {
            // If validation was sucesful
             let res = add_quiz_to_db(&pool, &obj).await;
            match res {
                Ok(()) => println!("Saved quiz on db"),
                Err(e) => return Err(e.to_string()),
            }
        },
        Err(e) => {
            // TODO we need to inform somehow user that he messed up creating a quizz
            return Err(String::from("You messed up filling up the quiz creator form lol"))
        },
    }
    Ok(())
}

#[tauri::command]
pub async fn get_quizzes_metadata(pool: State<'_, SqlitePool>) -> Result<Vec<QuizMetadata>, String> {
    match get_quizzes_metadata_db(&pool).await {
        Ok(o) => return Ok(o),
        Err(e) => return Err(e.to_string()),
    }
}