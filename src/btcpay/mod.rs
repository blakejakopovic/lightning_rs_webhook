use hex::encode;
use ring::{hmac, hmac::Key, hmac::HMAC_SHA256};

pub mod btcpay_middleware;
pub mod btcpay_models;

pub fn verify_signature(payload_body: &str, secret_token: &str, signature_header: &str) -> bool {
    let expected_signature = hmac_sha256(payload_body.as_bytes(), secret_token.as_bytes());
    let actual_signature = signature_header.trim_start_matches("sha256=");

    actual_signature == expected_signature
}

fn hmac_sha256(message: &[u8], secret_key: &[u8]) -> String {
    let key = Key::new(HMAC_SHA256, secret_key);
    let signature = hmac::sign(&key, message);
    encode(signature.as_ref())
}
