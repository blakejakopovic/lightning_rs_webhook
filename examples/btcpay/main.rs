#[macro_use]
extern crate log;
use actix_web::{App, post, HttpServer, middleware::Logger, web, HttpResponse, Responder};
use anyhow::Result;
use deadpool_postgres::{Pool as PGPool};
use dotenv::dotenv;
use lightning_rs_webhook::db::pg_pool_from_url;
use lightning_rs_webhook::btcpay::{btcpay_middleware, btcpay_models::WebhookPayload};
use lightning_rs_webhook::routes;

#[allow(unused_variables)]
async fn webhook_handler(pg_pool: &PGPool, payload: WebhookPayload) -> Result<()> {
    match payload {

        // Triggers when an invoice is considered settled and the merchant can proceed with the order's delivery. The
        // invoice now has enough confirmations on the blockchain (if paid on-chain) according to your store's configuration.
        WebhookPayload::InvoiceSettled(event) => {
            debug!("InvoiceSettled Event: {event:?}");

            // let pg_conn = pg_pool.get().await?;

            // let invoice_id = event.invoice_id;

            // // TODO: Lookup the invoice_id to lookup the invoice pubkey and content_id metadata
            // let pubkey = "";
            // let content_id = "";

            // pg_conn.execute("
            //     UPDATE access_table
            //     SET (pubkey, content_id, paid)
            //     VALUES ($1, $2, true)
            //     ON CONFLICT (pubkey, content_id)
            //     DO NOTHING;
            // ", &[&pubkey, &content_id]).await?;

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
