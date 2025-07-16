use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Media {
    pub sha1: Vec<u8>,
    pub remote_id: String,
    pub local_reference: String,
    pub local_size: i32,
    pub key: Vec<u8>,
    pub meta_data: Vec<u8>,
    pub r#type: i32,
    pub state: i32,
    pub file_name: String,
    pub sort_order: i32,
    pub error_count: i32,
    pub version: i32,
    pub format: i32,
    pub error_message: String,
    pub local_bucket: String,
    pub liked: i32,
    pub hidden: i32,
    pub dirty: i32,
}