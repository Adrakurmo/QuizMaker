use std::{fs, path::Path};

use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

use crate::models::{quiz::Quiz, quizmetadata::QuizMetadata};

pub async fn init_db() -> Pool<Sqlite> {
    // TMP FOR TEST TIME LOL
    let db_path = "target/quiz.db";

    if !Path::new(db_path).exists() {
        fs::File::create(db_path).expect("Couldn't create db file :c");
    }

    let pool = SqlitePoolOptions::new()
        .connect("sqlite://target/quiz.db")
        .await
        .expect("Couldn't connect with db");

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .expect("Couldn't turn on FK");

    // #########################################################################################
    // QUIZZES
    // #########################################################################################
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS quizzes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        created_at TEXT NOT NULL
        );"
    )
        .execute(&pool)
        .await
        .expect("Somethin went wrong with creating table quizzes");

    // #########################################################################################
    // QUESTIONS
    // #########################################################################################
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS questions (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        question_text TEXT NOT NULL,
        quiz_id INTEGER NOT NULL, -- FK
        FOREIGN KEY (quiz_id) REFERENCES quizzes(id) ON DELETE CASCADE
        );"
    )
        .execute(&pool)
        .await
        .expect("Somethin went wrong with creating table questions");

    // #########################################################################################
    // ANSWERS
    // #########################################################################################
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS answers (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        answer_text TEXT NOT NULL,
        question_id INTEGER NOT NULL, -- FK
        is_correct BOOLEAN NOT NULL,
        FOREIGN KEY (question_id) REFERENCES questions(id) ON DELETE CASCADE
        );"
    )
        .execute(&pool)
        .await
        .expect("Somethin went wrong with creating table answers");


    pool
}


pub async fn add_quiz_to_db(pool: &Pool<Sqlite>, quiz: &Quiz) -> Result<(), sqlx::Error> {
    let quiz_insert = sqlx::query(
        "INSERT INTO quizzes (title, created_at) VALUES (?, datetime('now'))"
    )
        .bind(&quiz.title)
        .execute(pool)
        .await?;

    let quiz_id = quiz_insert.last_insert_rowid();

    for question in quiz.questions.iter() {
        let question_insert = sqlx::query (
            "INSERT INTO questions (question_text, quiz_id) VALUES (?, ?)"
        )
            .bind(&question.text)
            .bind(&quiz_id)
            .execute(pool)
            .await?;

        let question_id = question_insert.last_insert_rowid();

        for answer in question.answers.iter() {
            sqlx::query(
                "INSERT INTO answers (answer_text, is_correct, question_id) VALUES(?,?,?)"
            )
                .bind(&answer.text)
                .bind(&answer.is_correct)
                .bind(&question_id)
                .execute(pool)
                .await?;
        }
    } 

    Ok(())
}


pub async fn get_quizzes_metadata_db(pool: &Pool<Sqlite>) -> Result<Vec<QuizMetadata>, sqlx::Error> {
    let quizzes_md = sqlx::query_as::<sqlx::Sqlite, QuizMetadata>("
        SELECT id, title, created_at FROM quizzes
    ")
        .fetch_all(pool)
        .await?;

    Ok(quizzes_md)
}