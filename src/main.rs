// use serde::{Deserialize, Serialize};

// TODO: import log, pretty_env_logger, dotenv, and PgPoolOptions

#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use std::sync::Arc;

// use log::*;
// use pretty_env_logger::*;
use dotenvy::*;
use sqlx::postgres::PgPoolOptions;

use axum::{routing::{delete, get, post},Router};

mod models;
mod handlers;
use handlers::*;
mod persistance;

use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};


#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

#[tokio::main]

async fn main() {

    pretty_env_logger::init();
    dotenv().ok();
   
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");
        // .connect(dotenvy::var("DATABASE_URL").expect("Error: ENV VAR -DATABASE_URL-").as_str())
        // .await.expect("Error: No live Database");
    
        // let recs= sqlx::query("SELECT * FROM questions").fetch_all(&pool).await.expect("Error in Query");
        // let recs= sqlx::query!("SELECT * FROM questions").fetch_all(&pool).await.expect("Error in Query");
    
    // info!("********* Question Records *********");
    // info!("{:?}",recs);

    // let questions_dao = todo!(); // create a new instance of QuestionsDaoImpl passing in `pool` (use the clone method)
    let questions_dao = Arc::new(QuestionsDaoImpl::new(pool.clone()));
    // let answers_dao = todo!(); // create a new instance of AnswersDaoImpl passing in `pool`
    let answers_dao = Arc::new(AnswersDaoImpl::new(pool));
    // let app_state = todo!(); // create a new instance of AppState
    
    let app_state = AppState {
        questions_dao,
        answers_dao
    };
    

    let app = Router::new()
        .route("/", get(root))
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
        // The with_state method allows us to add state to the state managed by this instance of Axum. Then we can use this state in the handlers.
        .with_state(app_state); // pass in `app_state` as application state.


    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

async fn root() -> &'static str {
    "Welcome to Clone of StackOverFlow API!"
}