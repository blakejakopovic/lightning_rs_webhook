use serde::{Serialize, Deserialize};
// https://github.com/AE9999/btcpay-rust/tree/master/src/models

// TODO: Add other webhook events
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebhookPayload {
    InvoiceSettled(WebhookInvoiceSettledEvent),
    InvoicePaymentSettled(WebhookInvoicePaymentSettledEvent),
    InvoiceReceivedPayment(WebhookInvoiceReceivedPaymentEvent),
    Unsupported
}

/// Triggers when an invoice is considered settled and the merchant can proceed with the order's delivery. The
/// invoice now has enough confirmations on the blockchain (if paid on-chain) according to your store's configuration.
// https://docs.btcpayserver.org/API/Greenfield/v1/#operation/Webhook_InvoiceSettled
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebhookInvoiceSettledEvent {
    /// The delivery id of the webhook
    #[serde(rename = "deliveryId", skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    /// The id of the webhook
    #[serde(rename = "webhookId", skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    /// If this delivery is a redelivery, the is the delivery id of the original delivery.
    #[serde(rename = "originalDeliveryId", skip_serializing_if = "Option::is_none")]
    pub original_delivery_id: Option<String>,
    /// True if this delivery is a redelivery
    #[serde(rename = "isRedelivery", skip_serializing_if = "Option::is_none")]
    pub is_redelivery: Option<bool>,
    /// The type of this event, current available are `InvoiceCreated`, `InvoiceReceivedPayment`, `InvoiceProcessing`, `InvoiceExpired`, `InvoiceSettled`, `InvoiceInvalid`, and `InvoicePaymentSettled`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The timestamp when this delivery has been created
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f32>,
    /// The store id of the invoice's event
    #[serde(rename = "storeId", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The invoice id of the invoice's event
    #[serde(rename = "invoiceId", skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Whether the invoice have been manually marked as confirmed
    #[serde(rename = "manuallyMarked", skip_serializing_if = "Option::is_none")]
    pub manually_marked: Option<bool>,
    /// Whether this invoice has received more money than expected
    #[serde(rename = "overPaid", skip_serializing_if = "Option::is_none")]
    pub over_paid: Option<bool>,
}

impl WebhookInvoiceSettledEvent {
    /// Callback sent if the `type` is `InvoiceSettled`
    pub fn new() -> WebhookInvoiceSettledEvent {
        WebhookInvoiceSettledEvent {
            delivery_id: None,
            webhook_id: None,
            original_delivery_id: None,
            is_redelivery: None,
            _type: None,
            timestamp: None,
            store_id: None,
            invoice_id: None,
            manually_marked: None,
            over_paid: None,
        }
    }
}

/// An payment relating to an invoice has settled
// https://docs.btcpayserver.org/API/Greenfield/v1/#operation/Webhook_InvoicePaymentSettled
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebhookInvoicePaymentSettledEvent {
    /// The delivery id of the webhook
    #[serde(rename = "deliveryId", skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    /// The id of the webhook
    #[serde(rename = "webhookId", skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    /// If this delivery is a redelivery, the is the delivery id of the original delivery.
    #[serde(rename = "originalDeliveryId", skip_serializing_if = "Option::is_none")]
    pub original_delivery_id: Option<String>,
    /// True if this delivery is a redelivery
    #[serde(rename = "isRedelivery", skip_serializing_if = "Option::is_none")]
    pub is_redelivery: Option<bool>,
    /// The type of this event, current available are `InvoiceCreated`, `InvoiceReceivedPayment`, `InvoiceProcessing`, `InvoiceExpired`, `InvoiceSettled`, `InvoiceInvalid`, and `InvoicePaymentSettled`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The timestamp when this delivery has been created
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f32>,
    /// The store id of the invoice's event
    #[serde(rename = "storeId", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The invoice id of the invoice's event
    #[serde(rename = "invoiceId", skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Whether this payment has been sent after expiration of the invoice
    #[serde(rename = "afterExpiration", skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<bool>,
    /// What payment method was used for this payment
    #[serde(rename = "paymentMethod", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(rename = "payment", skip_serializing_if = "Option::is_none")]
    pub payment: Option<Payment>,
}

impl WebhookInvoicePaymentSettledEvent {
    /// Callback sent if the `type` is `InvoicePaymentSettled`
    pub fn new() -> WebhookInvoicePaymentSettledEvent {
        WebhookInvoicePaymentSettledEvent {
            delivery_id: None,
            webhook_id: None,
            original_delivery_id: None,
            is_redelivery: None,
            _type: None,
            timestamp: None,
            store_id: None,
            invoice_id: None,
            after_expiration: None,
            payment_method: None,
            payment: None,
        }
    }
}

/// An invoice received a payment
// https://docs.btcpayserver.org/API/Greenfield/v1/#operation/Webhook_InvoiceReceivedPayment
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebhookInvoiceReceivedPaymentEvent {
    /// The delivery id of the webhook
    #[serde(rename = "deliveryId", skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    /// The id of the webhook
    #[serde(rename = "webhookId", skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    /// If this delivery is a redelivery, the is the delivery id of the original delivery.
    #[serde(rename = "originalDeliveryId", skip_serializing_if = "Option::is_none")]
    pub original_delivery_id: Option<String>,
    /// True if this delivery is a redelivery
    #[serde(rename = "isRedelivery", skip_serializing_if = "Option::is_none")]
    pub is_redelivery: Option<bool>,
    /// The type of this event, current available are `InvoiceCreated`, `InvoiceReceivedPayment`, `InvoiceProcessing`, `InvoiceExpired`, `InvoiceSettled`, `InvoiceInvalid`, and `InvoicePaymentSettled`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The timestamp when this delivery has been created
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f32>,
    /// The store id of the invoice's event
    #[serde(rename = "storeId", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The invoice id of the invoice's event
    #[serde(rename = "invoiceId", skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Whether this payment has been sent after expiration of the invoice
    #[serde(rename = "afterExpiration", skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<bool>,
    /// What payment method was used for this payment
    #[serde(rename = "paymentMethod", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(rename = "payment", skip_serializing_if = "Option::is_none")]
    pub payment: Option<Payment>,
}

impl WebhookInvoiceReceivedPaymentEvent {
    /// Callback sent if the `type` is `InvoiceReceivedPayment`
    pub fn new() -> WebhookInvoiceReceivedPaymentEvent {
        WebhookInvoiceReceivedPaymentEvent {
            delivery_id: None,
            webhook_id: None,
            original_delivery_id: None,
            is_redelivery: None,
            _type: None,
            timestamp: None,
            store_id: None,
            invoice_id: None,
            after_expiration: None,
            payment_method: None,
            payment: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Payment {
    /// A unique identifier for this payment
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The date the payment was recorded
    #[serde(rename = "receivedDate", skip_serializing_if = "Option::is_none")]
    pub received_date: Option<f32>,
    /// The value of the payment
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The fee paid for the payment
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    /// The destination the payment was made to
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

impl Payment {
    pub fn new() -> Payment {
        Payment {
            id: None,
            received_date: None,
            value: None,
            fee: None,
            status: None,
            destination: None,
        }
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "Invalid")]
    Invalid,
    #[serde(rename = "Processing")]
    Processing,
    #[serde(rename = "Settled")]
    Settled,
}

impl ToString for PaymentStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Invalid => String::from("Invalid"),
            Self::Processing => String::from("Processing"),
            Self::Settled => String::from("Settled"),
        }
    }
}

impl Default for PaymentStatus {
    fn default() -> PaymentStatus {
        Self::Invalid
    }
}
