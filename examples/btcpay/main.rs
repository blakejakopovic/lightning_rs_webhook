#![allow(unused_imports)]
#[macro_use]
extern crate log;
use actix_web::{App, post, HttpServer, middleware::Logger, web, HttpResponse, Responder};
use anyhow::Result;
use deadpool_postgres::{Pool as PGPool};
use dotenv::dotenv;
use lightning_rs_webhook::btcpay::{btcpay_middleware, get_invoice_data, WebhookPayload};
use lightning_rs_webhook::db::pg_pool_from_url;
use lightning_rs_webhook::error::ServiceError;
use lightning_rs_webhook::routes;
use serde_json::Value;

#[allow(unused_variables)]
async fn webhook_handler(pg_pool: &PGPool, payload: WebhookPayload) -> Result<()> {
    match payload {

        // Triggers when an invoice is considered settled and the merchant can proceed with the order's
        // delivery. The invoice now has enough confirmations on the blockchain (if paid on-chain) according
        // to your store's configuration.
        WebhookPayload::InvoiceSettled(event) => {
            debug!("InvoiceSettled Event: {event:?}");

            // Note: The example below is expecting that the invoice was created with posData
            //       that includes a Nostr pubkey and content_id. Your needs may vary.
            //
            //        There are two alternatives for invoice processing and database updating
            //        as examples. You only need to pick one of each, or write your own.

            // Event Processing Approach 1. - using webhook data
            // let pos_data = event
            //     .metadata.ok_or(ServiceError::InternalError)?
            //     .pos_data.ok_or(ServiceError::InternalError)?;

            // let pubkey = pos_data
            //     .get("pubkey").ok_or(ServiceError::BadClientData)?
            //     .as_str().ok_or(ServiceError::BadClientData)?
            //     .to_string();

            // let content_id = pos_data
            //     .get("content_id").ok_or(ServiceError::BadClientData)?
            //     .as_str().ok_or(ServiceError::BadClientData)?
            //     .to_string();

            // Event Processing Approach 2. - using REST API to fetch full invoice data record
            //
            // Ensure store_id and invoice_id are populated
            // let store_id = event.store_id.ok_or(ServiceError::BadClientData)?;
            // let invoice_id = event.invoice_id.ok_or(ServiceError::BadClientData)?;

            // // Fetch the invoice via the API
            // let invoice_data = get_invoice_data(&store_id, &invoice_id)
            //     .await
            //     .map_err(|_| ServiceError::InternalError)?;

            // debug!("{invoice_data:?}");

            // // Validate Invoice response
            // let invoice_id = invoice_data.id.ok_or(ServiceError::InternalError)?;

            // let pos_data_json = invoice_data
            //     .metadata
            //     .ok_or(ServiceError::InternalError)?
            //     .pos_data
            //     .ok_or(ServiceError::InternalError)?;

            // let pos_data: Value = serde_json::from_str(&pos_data_json)
            //     .map_err(|_| ServiceError::InternalError)?;

            // // Extract what we need to update the database
            // // Note: Since we are populating the posData values in BTCPay server, we can skip validation
            // //       here - unless you are risk adverse.
            // let pubkey = pos_data.get("pubkey").ok_or(ServiceError::InternalError)?.to_string();
            // let content_id = pos_data.get("content_id").ok_or(ServiceError::InternalError)?.to_string();


            // Database Approach 1. - single query
            // This query will insert the pubkey into the identities table if not found, before
            // inserting the payment record. Noting, if the content_id is not found, it's an NOOP returning 200
            // let pg_conn = pg_pool.get().await?;

            // let result = pg_conn.execute("
            //     WITH selected_identity AS (
            //       SELECT id
            //       FROM identities
            //       WHERE pubkey = $1
            //       LIMIT 1
            //     ), inserted_identity AS (
            //       INSERT INTO identities (pubkey)
            //       SELECT $1
            //       WHERE NOT EXISTS (SELECT 1 FROM selected_identity)
            //       RETURNING id
            //     )
            //     INSERT INTO payments (identity_id, content_id)
            //     SELECT COALESCE(selected_identity.id, inserted_identity.id), content.id
            //     FROM selected_identity
            //     FULL JOIN inserted_identity ON true
            //     JOIN content ON content.content_id = $2
            //     ON CONFLICT (identity_id, content_id) DO NOTHING;
            // ", &[&pubkey, &content_id]).await?;

            // Database Approach 2. - using transactions
            // let mut pg_conn = pg_pool.get().await?;
            // let pg_trans = pg_conn.transaction().await?;

            // // Ensure the identity pubkey record exists
            // let db_identity = pg_trans.query_one("
            //     WITH new_i AS(
            //         INSERT INTO identities (pubkey)
            //                VALUES ($1)
            //         ON CONFLICT DO NOTHING
            //         RETURNING id
            //     )
            //     SELECT id FROM new_i
            //     UNION
            //     SELECT id FROM identities WHERE pubkey=$1",
            //     &[&pubkey]).await?;

            // let identity_id: i32 = db_identity.get(0);

            // // Insert or ignore if existing payment row exists
            // pg_trans.execute("
            //     INSERT INTO payments (
            //       identity_id,
            //       content_id
            //     )
            //     VALUES (
            //       $1,
            //       (select id from content where content_id = $2)
            //     )
            //     ON CONFLICT (identity_id, content_id) DO NOTHING;
            // ", &[&identity_id, &content_id]).await?;

            // pg_trans.commit().await?;

            Ok(())
        },

        // An payment relating to an invoice has settled
        WebhookPayload::InvoicePaymentSettled(event) => {
            debug!("InvoicePaymentSettled Event: {event:?}");
            Ok(())
        },

        // An invoice received a payment
        WebhookPayload::InvoiceReceivedPayment(event) => {
            debug!("InvoiceReceivedPayment Event: {event:?}");
            Ok(())
        },

        // An invoice expired
        WebhookPayload::InvoiceExpired(event) => {
            debug!("InvoiceExpired Event: {event:?}");
            Ok(())
        },

        // An invoice became invalid
        WebhookPayload::InvoiceInvalid(event) => {
            debug!("InvoiceInvalid Event: {event:?}");
            Ok(())
        },

        // Triggers when an invoice is fully paid, but doesn't have the required amount of confirmations
        // on the blockchain yet according to your store's settings.
        WebhookPayload::InvoiceProcessing(event) => {
            debug!("InvoiceProcessing Event: {event:?}");
            Ok(())
        },

        // A new invoice has been created
        WebhookPayload::InvoiceCreated(event) => {
            debug!("InvoiceCreated Event: {event:?}");
            Ok(())
        },

        // Any unhandled webhook events - return ok, as we don't have any logic for them yet
        WebhookPayload::Unsupported => {
            debug!("Unsupported Event");
            Ok(())
        },
    }
}

// Note: This is scoped to inject the middleware - /btcpay/webhook is the full path
#[post("/webhook")]
pub async fn btcpay_webhook_handler(payload: web::Json<WebhookPayload>, app_data: web::Data<AppData>) -> impl Responder {

    match webhook_handler(&app_data.pg_pool, payload.into_inner()).await  {
        Ok(_) => {
            // Webhook caller is expecting a 200 response
            HttpResponse::Ok().finish()
        },
        Err(err) => {
            error!("Error: {err:?}");
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}

pub struct AppData {
    pub pg_pool: PGPool,
}

#[actix_web::main]
async fn main() -> Result<()> {

    env_logger::init();

    dotenv().ok();

    let host: String = std::env::var("HOST").expect("HOST must be set");
    let port: String = std::env::var("PORT").expect("PORT must be set");

    let pg_address: String = std::env::var("POSTGRES_ADDRESS").expect("POSTGRES_ADDRESS must be set.");
    let pg_pool = pg_pool_from_url(&pg_address)?;

    println!("Running BTCPay Webhook Server on {host}:{port}");

    HttpServer::new(move || {

        let logger = Logger::default();

        let app_data = AppData {
            pg_pool: pg_pool.clone()
        };

        App::new()
            .wrap(logger)
            .app_data(web::Data::new(app_data))
            .service(routes::health_handler)
            .service(
                web::scope("/btcpay")
                    .wrap(btcpay_middleware::BTCPayHeaderVerify)
                    .service(btcpay_webhook_handler)
            )
    })
    .bind(format!("{host}:{port}"))?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use actix_web::{http::header::ContentType, test, App};
    use std::env;
    use super::*;

    #[actix_web::test]
    async fn test_health_get() {
        let app = test::init_service(App::new().service(super::routes::health_handler)).await;

        let req = test::TestRequest::default()
            .uri("/health")
            .insert_header(ContentType::html())
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_btcpay_webhook_post_valid() {
        // Note: Dummy data
        env::set_var("BTCPAY_WEBHOOK_SECRET", "Y6Tio3rXRT4dGqpk43GvBPK9fHQ");
        env::set_var("POSTGRES_ADDRESS", "postgresql://postgres:postgres@localhost:5433/postgres");

        let pg_address: String = std::env::var("POSTGRES_ADDRESS").expect("POSTGRES_ADDRESS must be set.");
        let pg_pool = pg_pool_from_url(&pg_address).unwrap();

        let app_data = AppData {
            pg_pool: pg_pool.clone()
        };

        let app = test::init_service(App::new()
                                     .app_data(web::Data::new(app_data))
                                     .service(
                                          web::scope("/btcpay")
                                              .wrap(btcpay_middleware::BTCPayHeaderVerify)
                                              .service(btcpay_webhook_handler)
                                      )).await;
        let req = test::TestRequest::default()
            .uri("/btcpay/webhook")
            .method(actix_http::Method::POST)
            .insert_header((super::btcpay_middleware::BTCPAY_SIG_HEADER, "sha256=237906b0175aa4de911eba91ec0791e7482333a5de9f81a179442fc602b0d1be"))
            // Note: We cannot use .set_json as it will mangle the payload and cause a bad signature check
            .insert_header(ContentType::json())
            .set_payload(r#"{
  "manuallyMarked": false,
  "deliveryId": "WZbyGsmWGZvYjRsYCH7Vmt",
  "webhookId": "AT7ogqNzXkjf12sLVWPDNS",
  "originalDeliveryId": "WZbyGsmWGZvYjRsYCH7Vmt",
  "isRedelivery": false,
  "type": "InvoiceSettled",
  "timestamp": 1683049755,
  "storeId": "BJKmPvug3KHVWyu1ECEiAstAQXFjJD1fX87EcgEhHVLT",
  "invoiceId": "6wmoR7p5UFVzCYuwyiViKX",
  "metadata": {
    "orderId": "23",
    "physical": false
  }
}"#)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_btcpay_webhook_post_missing_sig_header() {
        // Note: Dummy data
        env::set_var("BTCPAY_WEBHOOK_SECRET", "Y6Tio3rXRT4dGqpk43GvBPK9fHQ");
        env::set_var("POSTGRES_ADDRESS", "postgresql://postgres:postgres@localhost:5433/postgres");

        let pg_address: String = std::env::var("POSTGRES_ADDRESS").expect("POSTGRES_ADDRESS must be set.");
        let pg_pool = pg_pool_from_url(&pg_address).unwrap();

        let app_data = AppData {
            pg_pool: pg_pool.clone()
        };

        let app = test::init_service(App::new()
                                     .app_data(web::Data::new(app_data))
                                     .service(
                                          web::scope("/btcpay")
                                              .wrap(btcpay_middleware::BTCPayHeaderVerify)
                                              .service(btcpay_webhook_handler)
                                      )).await;
        let req = test::TestRequest::default()
            .uri("/btcpay/webhook")
            .method(actix_http::Method::POST)
            // Note: We cannot use .set_json as it will mangle the payload and cause a bad signature check
            .insert_header(ContentType::json())
            .set_payload(r#"{
  "manuallyMarked": false,
  "deliveryId": "WZbyGsmWGZvYjRsYCH7Vmt",
  "webhookId": "AT7ogqNzXkjf12sLVWPDNS",
  "originalDeliveryId": "WZbyGsmWGZvYjRsYCH7Vmt",
  "isRedelivery": false,
  "type": "InvoiceSettled",
  "timestamp": 1683049755,
  "storeId": "BJKmPvug3KHVWyu1ECEiAstAQXFjJD1fX87EcgEhHVLT",
  "invoiceId": "6wmoR7p5UFVzCYuwyiViKX",
  "metadata": {
    "orderId": "23",
    "physical": false
  }
}"#)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_btcpay_webhook_post_invalid_sig_header() {
        // Note: Dummy data
        env::set_var("BTCPAY_WEBHOOK_SECRET", "Y6Tio3rXRT4dGqpk43GvBPK9fHQ");
        env::set_var("POSTGRES_ADDRESS", "postgresql://postgres:postgres@localhost:5433/postgres");

        let pg_address: String = std::env::var("POSTGRES_ADDRESS").expect("POSTGRES_ADDRESS must be set.");
        let pg_pool = pg_pool_from_url(&pg_address).unwrap();

        let app_data = AppData {
            pg_pool: pg_pool.clone()
        };

        let app = test::init_service(App::new()
                                     .app_data(web::Data::new(app_data))
                                     .service(
                                          web::scope("/btcpay")
                                              .wrap(btcpay_middleware::BTCPayHeaderVerify)
                                              .service(btcpay_webhook_handler)
                                      )).await;
        let req = test::TestRequest::default()
            .uri("/btcpay/webhook")
            .method(actix_http::Method::POST)
            .insert_header((super::btcpay_middleware::BTCPAY_SIG_HEADER, "sha256=BADSIG"))
            // Note: We cannot use .set_json as it will mangle the payload and cause a bad signature check
            .insert_header(ContentType::json())
            .set_payload(r#"{
  "manuallyMarked": false,
  "deliveryId": "WZbyGsmWGZvYjRsYCH7Vmt",
  "webhookId": "AT7ogqNzXkjf12sLVWPDNS",
  "originalDeliveryId": "WZbyGsmWGZvYjRsYCH7Vmt",
  "isRedelivery": false,
  "type": "InvoiceSettled",
  "timestamp": 1683049755,
  "storeId": "BJKmPvug3KHVWyu1ECEiAstAQXFjJD1fX87EcgEhHVLT",
  "invoiceId": "6wmoR7p5UFVzCYuwyiViKX",
  "metadata": {
    "orderId": "23",
    "physical": false
  }
}"#)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}
