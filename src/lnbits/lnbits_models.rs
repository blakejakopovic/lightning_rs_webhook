use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebhookPayload {
    Payment(PaymentEvent),
    Unsupported
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentEvent {
    pub checking_id: String,
    pub pending: bool,
    pub amount: u64,
    pub fee: u64,
    pub memo: String,
    pub time: u64,
    pub bolt11: String,
    pub preimage: String,
    pub payment_hash: String,
    pub expiry: u64,
    pub extra: serde_json::Value,
    pub wallet_id: String,
    pub webhook: String,
    pub webhook_status: Option<u64>,
}
