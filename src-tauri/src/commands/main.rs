use crate::{models::question::Question, services::quiz_service::validate_quiz_data};



#[tauri::command]
pub async fn add_quiz(quiz_name: String, questions: Vec<Question>) -> Result<(), String> {
    match validate_quiz_data(&quiz_name, questions) {
        Ok(obj) => {
            // If validation was sucesful
            
        },
        Err(e) => {
            // TODO we need to inform somehow user that he messed up creating a quizz
            return Err(String::from("You messed up filling up the quiz creator form lol"))
        },
    }
    Ok(())
}

