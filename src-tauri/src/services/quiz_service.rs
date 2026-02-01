use crate::models::{question::{self, Question}, quiz::Quiz};

const TITLE_MAX_LEN: usize = 60;

pub fn validate_quiz_data(name: &String, mut _questions:  Vec<Question>) -> Result<Quiz, String> {
    let new_name: String = name.chars().take(TITLE_MAX_LEN).collect();

    for q in _questions.iter_mut() {
        // Removing empty answers
        q.answers.retain(|a| !a.text.is_empty());
    }
    // Removing empty answers and questions without text
    _questions.retain(|q| !q.answers.is_empty() && !q.text.is_empty());

    if _questions.is_empty() {
        return Err(String::from("No question for this quizz"))
    }

    let new_quiz = Quiz {
        id: 0,
        title: new_name,
        created_at: String::from(""),
        questions: _questions
    };


    Ok(new_quiz)
}

