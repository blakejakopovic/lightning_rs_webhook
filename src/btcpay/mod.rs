use anyhow::Result;
use hex::encode;
use lazy_static::lazy_static;
use ring::{hmac, hmac::Key, hmac::HMAC_SHA256};
use serde::{Serialize, Deserialize};

pub mod btcpay_middleware;
pub mod btcpay_models;

// Use our custom types here to support the metadata key and fix physical being a bool, instead of a string
pub use btcpay_models::{
  WebhookInvoiceSettledEvent,
  WebhookInvoiceCreatedEvent
};
pub use btcpay_client::models::{
  WebhookInvoicePaymentSettledEvent,
  WebhookInvoiceReceivedPaymentEvent,
  WebhookInvoiceExpiredEvent,
  WebhookInvoiceInvalidEvent,
  WebhookInvoiceProcessingEvent,
  InvoiceData
};


lazy_static! {
    static ref BTCPAY_HOST: String = std::env::var("BTCPAY_HOST").expect("BTCPAY_HOST must be set");
    static ref BTCPAY_API_KEY: String = std::env::var("BTCPAY_API_KEY").expect("BTCPAY_API_KEY must be set");
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebhookPayload {
    InvoiceSettled(WebhookInvoiceSettledEvent),
    InvoicePaymentSettled(WebhookInvoicePaymentSettledEvent),
    InvoiceReceivedPayment(WebhookInvoiceReceivedPaymentEvent),
    InvoiceExpired(WebhookInvoiceExpiredEvent),
    InvoiceInvalid(WebhookInvoiceInvalidEvent),
    InvoiceProcessing(WebhookInvoiceProcessingEvent),
    InvoiceCreated(WebhookInvoiceCreatedEvent), // Note: Not yet part of btcpay-client crate
    Unsupported
}

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

pub async fn get_invoice_data(store_id: &str, invoice_id: &str) -> Result<InvoiceData> {

    // Build the URL for the API endpoint
    let url = format!("{}/api/v1/stores/{store_id}/invoices/{invoice_id}", *BTCPAY_HOST);

    // Send the GET request with the API key in the headers and deserialize the response JSON into an InvoiceData struct
    let response = reqwest::Client::new()
        .get(&url)
        .header(reqwest::header::AUTHORIZATION, format!("token {}", *BTCPAY_API_KEY))
        .send()
        .await?
        .json::<InvoiceData>()
        .await?;

    Ok(response)
}
