use chrono;

#[derive(Debug)]
struct Media {
    pub sha1: [&u8];
    pub ts: String;
    pub remote_id: String;
    pub local_reference: String;
    pub local_size: i32;
    pub key: [&u8];
    pub meta_data [&u8];
    pub type: i32;
    pub state: i32;
    pub updated_at: String;
    pub file_name: String;
    pub sort_order: i32;
    pub error_count: i32;
    pub version: i32;
    pub format: i32;
    pub error_message: String;
    pub local_bucket: String;
    pub liked: i32;
    pub hidden: i32;
    pub dirty: i32;
}