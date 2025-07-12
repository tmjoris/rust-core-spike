use std::error::Error;
use crate::model::media::Media;
use chrono;

async fn create(media: &Media, pool: &sqlx::PgPool)-> Result<(), Box<dyn Error>>{
    let ts = chrono::offset::Utc::now();
    let query = "INSERT INTO media (sha1, ts, remote_id, local_reference, sha1, ts, remote_id, local_reference, local_size, key, meta_data, type, state, updated_at, file_name, sort_order, error_count, version, format, error_message, local_bucket, liked, hidden, dirty)"

    sqlx::query_as!(media, query)
        .execute(pool)
        .await?;
    Ok(())
}