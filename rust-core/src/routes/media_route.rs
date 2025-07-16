use crate::models::media::Media;
use chrono;
use axum::{http::StatusCode, debug_handler, 
            response::{IntoResponse, Response}, 
            extract::{State, Json}};
use sqlx::{Error as SqlxError};

#[derive(Debug)]
pub enum AppError {
    Sqlx(SqlxError),
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> AppError{
        AppError::Sqlx(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Sqlx(sqlx_err) => {
                eprintln!("Database error: {}", sqlx_err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };
        (status, error_message).into_response()
    }
}

#[debug_handler]
//pub async fn create(media: for<'a> &'a Media, pool: for<'a> &'a sqlx::SqlitePool)-> Result<StatusCode, String>{
pub async fn create(
    State(pool): State<sqlx::SqlitePool>,
    Json(media): Json<Media>
    )-> Result<StatusCode, AppError> {
    let ts = chrono::offset::Utc::now().to_rfc3339();
    let query = "INSERT INTO media (sha1, ts, remote_id, local_reference, local_size, key, meta_data, type, state, updated_at, file_name, sort_order, error_count, version, format, error_message, local_bucket, liked, hidden, dirty) 
    VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

    let result = sqlx::query(query)
        .bind(&media.sha1)
        .bind(ts)
        .bind(&media.remote_id)
        .bind(&media.local_reference)
        .bind(&media.local_size)
        .bind(&media.key)
        .bind(&media.meta_data)
        .bind(&media.r#type)
        .bind(&media.state)
        .bind(&media.updated_at)
        .bind(&media.file_name)
        .bind(&media.sort_order)
        .bind(&media.error_count)
        .bind(&media.version)
        .bind(&media.format)
        .bind(&media.error_message)
        .bind(&media.local_bucket)
        .bind(&media.liked)
        .bind(&media.hidden)
        .bind(&media.dirty)
        .execute(&pool)
        .await;
    match result {
        Ok(_) =>{
            println!("Successfully created media");
            Ok(StatusCode::CREATED)}
        Err(e) => {
            eprintln!("Failed to create media: {}", e);
            Err(AppError::Sqlx(e))
        }
    }
}