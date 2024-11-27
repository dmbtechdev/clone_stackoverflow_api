use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        // todo!(); // return an instance of QuestionsDaoImpl
        Self { db }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        // Make a database query to insert a new question.
        // Here is the SQL query:
        // ```
        // INSERT INTO questions ( title, description )
        // VALUES ( $1, $2 )
        // RETURNING *
        // ```
        // If executing the query results in an error, map that error to
        // the`DBError::Other` error and early return from this function.
        // let query = format!("INSERT INTO questions ( {}, {} ) VALUES ( $1, $2 ) RETURNING *",question.title, question.description);
        // let record= sqlx::query(&query)
        //     .bind(&question.title)
        //     .bind(&question.description)
        //     .fetch_one(&self.db)
        //     .await
        //     .map_err(|err| DBError::Other(err.to_string().into()))?;

        let record = sqlx::query!(
                r#"INSERT INTO questions ( title, description ) VALUES ( $1, $2 ) RETURNING * "#,
                question.title,
                question.description
            )
            .fetch_one(&self.db)
            .await
            .map_err(|err| DBError::Other(err.to_string().into()))?;

        // Populate the QuestionDetail fields using `record`.
        Ok(QuestionDetail {
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            created_at: record.created_at.to_string(),
        })

    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        // Use the `sqlx::types::Uuid::parse_str` method to parse `question_uuid` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        // let uuid = todo!();
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid)
            .map_err(|err| DBError::InvalidUUID(err.to_string().into()))?;
        
        // TODO: Make a database query to delete a question given the question uuid.
        // Here is the SQL query:
        // ```
        // DELETE FROM questions WHERE question_uuid = $1
        // ```
        // If executing the query results in an error, map that error
        // to a `DBError::Other` error and early return from this function.

        let record = sqlx::query!(
            r#"DELETE FROM questions WHERE question_uuid = $1 "#,
            uuid
        )
        .execute(&self.db)
        .await
        .map_err(|err| DBError::Other(err.to_string().into()))?;


        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        // Make a database query to get all questions.
        // Here is the SQL query:
        // ```
        // SELECT * FROM questions
        // ```
        // If executing the query results in an error, map that error
        // to a `DBError::Other` error and early return from this function.
        // let records = todo!();
        
        let records = sqlx::query!(
            r#"SELECT * FROM questions"#,
        )
        .fetch_all(&self.db)
        .await
        .map_err(|err| DBError::Other(err.to_string().into()))?;

        // Iterate over `records` and map each record to a `QuestionDetail` type
        // let questions = todo!();
        let questions = records.iter().map(|record|QuestionDetail{
            question_uuid:record.question_uuid.to_string(), 
            title: record.title.clone(), 
            description: record.description.clone(), 
            created_at: record.created_at.to_string()
        }).collect();


        Ok(questions)
    }
}