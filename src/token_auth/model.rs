use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub alg: String,
    pub typ: String,
}
#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub user_id: i32,
    pub role: String,
    pub exp: u64,
}