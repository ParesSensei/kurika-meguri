use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Hiragana {
    pub id: i32,
    pub character: String,
    pub romaji: String,
    pub row_group: String,
}

impl QuestionType {
    pub fn random() -> Self {
        if rand::random_range(0..2) == 0 {
            Self::HiraganaToRomaji
        } else {
            Self::RomajiToHiragana
        }
    }
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

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    HiraganaToRomaji,
    RomajiToHiragana,
}

#[derive(Serialize)]
pub struct QuestionResponse {
    pub id: u32,
    pub question: String,
    pub option: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Question {
    pub id: i32,
    pub character: String,
    pub romaji: String,
    pub question_type: QuestionType,
    pub option: Vec<String>,
    pub correct_answer: String,
}

#[test]
fn test_question() {
    let question = Question {
        id: 1,
        character: "あ".to_string(),
        romaji: "a".to_string(),
        question_type: QuestionType::HiraganaToRomaji,
        option: vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ],
        correct_answer: "a".to_string(),
    };

    println!(
        "what the correct answer of this question? {} {}",
        question.id, question.correct_answer
    );
}
