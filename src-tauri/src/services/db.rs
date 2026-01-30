use std::{fs, path::Path};

use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

pub type DbPool = Pool<Sqlite>;

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