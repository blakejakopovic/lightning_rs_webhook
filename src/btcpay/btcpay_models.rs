use serde::{Serialize, Deserialize};
use serde_json::Value;

// We needed to add the metadata property here as it was missing from the btcpay-client crate
// REF: https://github.com/AE9999/btcpay-rust
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
    /// The invoice metadata
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<InvoiceMetadata>>,
    /// Whether the invoice have been manually marked as confirmed
    #[serde(rename = "manuallyMarked", skip_serializing_if = "Option::is_none")]
    pub manually_marked: Option<bool>,
    /// Whether this invoice has received more money than expected
    #[serde(rename = "overPaid", skip_serializing_if = "Option::is_none")]
    pub over_paid: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvoiceMetadata {
    /// You can use this property to store the ID of an external system. We allow you to search in the invoice list based on this ID.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// You can use this property to store the URL to the order of an external system. This makes navigating to the order easier.
    #[serde(rename = "orderUrl", skip_serializing_if = "Option::is_none")]
    pub order_url: Option<String>,
    #[serde(rename = "posData", skip_serializing_if = "Option::is_none")]
    pub pos_data: Option<Value>, // Could not be rendered
    #[serde(rename = "buyerName", skip_serializing_if = "Option::is_none")]
    pub buyer_name: Option<String>,
    #[serde(rename = "buyerEmail", skip_serializing_if = "Option::is_none")]
    pub buyer_email: Option<String>,
    #[serde(rename = "buyerCountry", skip_serializing_if = "Option::is_none")]
    pub buyer_country: Option<String>,
    #[serde(rename = "buyerZip", skip_serializing_if = "Option::is_none")]
    pub buyer_zip: Option<String>,
    #[serde(rename = "buyerState", skip_serializing_if = "Option::is_none")]
    pub buyer_state: Option<String>,
    #[serde(rename = "buyerCity", skip_serializing_if = "Option::is_none")]
    pub buyer_city: Option<String>,
    #[serde(rename = "buyerAddress1", skip_serializing_if = "Option::is_none")]
    pub buyer_address1: Option<String>,
    #[serde(rename = "buyerAddress2", skip_serializing_if = "Option::is_none")]
    pub buyer_address2: Option<String>,
    #[serde(rename = "buyerPhone", skip_serializing_if = "Option::is_none")]
    pub buyer_phone: Option<String>,
    #[serde(rename = "itemDesc", skip_serializing_if = "Option::is_none")]
    pub item_desc: Option<String>,
    #[serde(rename = "itemCode", skip_serializing_if = "Option::is_none")]
    pub item_code: Option<String>,
    #[serde(rename = "physical", skip_serializing_if = "Option::is_none")]
    pub physical: Option<bool>,
    #[serde(rename = "taxIncluded", skip_serializing_if = "Option::is_none")]
    pub tax_included: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebhookInvoiceCreatedEvent {
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
    /// The type of this event, which is always "InvoiceCreated" for this struct
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The timestamp when this delivery has been created
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// The store id of the invoice's event
    #[serde(rename = "storeId", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The invoice id of the invoice's event
    #[serde(rename = "invoiceId", skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// The invoice metadata
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<InvoiceMetadata>>,
}
