use std::time::{
    SystemTime,
    UNIX_EPOCH
};

use base64::{
    engine::general_purpose::URL_SAFE_NO_PAD,
    Engine
};

use std::env;

use hmac::{Hmac, Mac};
use sha2::Sha256;

use super::model::{
    Payload,
    Header
};

type HmacSha256 = Hmac<Sha256>;

pub fn create_token(user_id: i32, role: String, exp_time_in_hours: u64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let exp = now + 60 * 60 * exp_time_in_hours;
    let payload = Payload {
        user_id: user_id,
        role: role,
        exp: exp
    };
    let header = Header {
        alg: "HS256".to_string(),
        typ: "JWT".to_string(),
    };
    let header_json = serde_json::to_string(&header).unwrap();
    let payload_json = serde_json::to_string(&payload).unwrap();
    let header_encoded = URL_SAFE_NO_PAD.encode(header_json);
    let payload_encoded = URL_SAFE_NO_PAD.encode(payload_json);
    let message = format!(
        "{}.{}",
        header_encoded,
        payload_encoded
    );
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not founded");
    let secret = secret.as_bytes();
    let mut mac = HmacSha256::new_from_slice(secret).expect("Hmac failed to create");
    mac.update(message.as_bytes());
    let signature = mac.finalize().into_bytes();
    let signature_encoded = URL_SAFE_NO_PAD.encode(signature);
    let token = format!("{}.{}.{}.",
        header_encoded,
        payload_encoded,
        signature_encoded
    );
    token
}