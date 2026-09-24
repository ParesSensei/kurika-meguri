use crate::models::hiragana::{Question, QuestionType};
use crate::{models::hiragana::Hiragana, state::AppState};
use axum::{Json, extract::State, http::StatusCode};
use tracing::{Level, event, info};

pub async fn get_all_hiragana_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Hiragana>>, (StatusCode, String)> {
    info!("get all hiragana route called");
    let hiragana = sqlx::query_as::<_, Hiragana>(
        "SELECT id, character, romaji, row_group
         FROM hiragana
         ORDER BY id ASC",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(hiragana))
}

pub async fn get_random_hiragana_handler(
    State(state): State<AppState>,
) -> Result<Json<Hiragana>, (StatusCode, String)> {
    event!(Level::INFO, "getting random hiragana route called");
    let ran_id = rand::random_range(1..=46);

    let hiragana = sqlx::query_as!(
        Hiragana,
        "SELECT id, character, romaji, row_group from hiragana WHERE id = $1",
        ran_id
    )
    .fetch_one(&state.pool)
    .await;

    let result = match hiragana {
        Ok(hiragana) => Ok(Json(hiragana)),
        Err(eror) => Err((StatusCode::INTERNAL_SERVER_ERROR, eror.to_string())),
    };
    result
}

pub async fn create_question(State(state): State<AppState>) -> Question {
    event!(Level::INFO, "created question");
    let ran_id = rand::random_range(1..=46);

    let hiragana = sqlx::query_as!(
        Hiragana,
        "select id, character, romaji, row_group from hiragana where id = $1",
        ran_id
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let option = sqlx::query_as!(
        Hiragana,
        "select id, character, romaji, row_group From hiragana where row_group = $1",
        hiragana.row_group
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    let options_romaji = option.clone()
        .into_iter()
        .map(|h| h.romaji)
        .collect();

    let option_hiragana: Vec<String> = option
        .into_iter()
        .map(|h| h.character)
        .collect();

    let hiragana_result = Question {
        id: hiragana.id,
        character: hiragana.character.clone(),
        romaji: hiragana.romaji.clone(),
        question_type: QuestionType::RomajiToHiragana,
        option: option_hiragana,
        correct_answer: hiragana.character.clone()
    };

    let romaji_result = Question {
        id: hiragana.id,
        character: hiragana.character.clone(),
        romaji: hiragana.romaji.clone(),
        question_type: QuestionType::HiraganaToRomaji,
        option: options_romaji,
        correct_answer: hiragana.romaji,
    };

    let question_type = QuestionType::random();
    match question_type {
        QuestionType::HiraganaToRomaji => {
            event!(Level::INFO, "question type HiraganaToRomaji");
            romaji_result
        }
        QuestionType::RomajiToHiragana => {
            event!(Level::INFO, "question type RomajiToHiragana");
            hiragana_result
        }
    }
}

pub async fn get_practice_question_handler(
    State(state): State<AppState>,
) -> Json<Question> {
    let question = create_question(State(state)).await;

    Json(question)
}

fn evaluate_answer(question: &Question, answer: String) -> bool {
    event!(Level::INFO, "evaluating answer");
    answer == question.correct_answer
}

#[tokio::test]
async fn question1() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    let pool = sqlx::PgPool::connect(&db_url)
        .await
        .unwrap();

    let state = AppState { pool };

    let question = create_question(State(state)).await;
    let answer = "ne".to_string();
    let result = evaluate_answer(&question, answer);
    println!("{:#?} {}", question, result);
}

// pub id: u32,
// pub character: String,
// pub romaji: String,
// pub question_type: QuestionType,
// pub option: Vec<String>,
// pub correct_answer: String,
