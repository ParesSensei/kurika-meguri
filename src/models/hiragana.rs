use serde::{Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct Hiragana {
    pub id: i32,
    pub character: String,
    pub romaji: String,
    pub row_group: String,
}

// Contoh menggunakan PostgreSQL ($1) dan makro query_as!
// let target_id = 42;
//
// let user = sqlx::query_as!(
//     User,
//     "SELECT id, name FROM users WHERE id = $1",
//     target_id
// )
// .fetch_one(&pool)
// .await?;

pub enum QuestionType {
    HiraganaToRomaji,
    RomajiToHiragana,
}

pub struct QuestionResponse {
    pub id : u32,
    pub question : String,
    pub option: Vec<String>,
}

pub struct Question {
    pub id: u32,
    pub character: String,
    pub romaji: String,
    pub question_type: QuestionType,
    pub option: Vec<String>,
    pub correct_answer: String,
}

#[test]
fn test_question() {
    let question = Question{
        id: 1,
        character: "あ".to_string(),
        romaji: "a".to_string(),
        question_type: QuestionType::HiraganaToRomaji,
        option: vec!["a".to_string(),"b".to_string(),"c".to_string(),"d".to_string()],
        correct_answer: "a".to_string(),
    };

    let q1 = println!("what the correct answer of this question? {} {}", question.id, question.correct_answer);
    q1
}