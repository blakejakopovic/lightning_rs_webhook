#[macro_use]
extern crate log;
use actix_web::{App, post, HttpServer, middleware::Logger, web, HttpResponse, Responder};
use anyhow::Result;
use deadpool_postgres::{Pool as PGPool};
use dotenv::dotenv;
use lightning_rs_webhook::db::pg_pool_from_url;
use lightning_rs_webhook::lnbits::lnbits_models::WebhookPayload;
use lightning_rs_webhook::routes;

#[allow(unused_variables)]
async fn webhook_handler(pg_pool: &PGPool, payload: WebhookPayload) -> Result<()> {
    match payload {

        // A webhook Payment Event
        WebhookPayload::Payment(payment) => {
            debug!("PaymentEvent: {payment:?}");

            // let pg_conn = pg_pool.get().await?;

            // TODO: This is only an example with poor error checking!
            // NOTE: It doesn't appear that LNbits has any HMAC check regarding webhook data. You may want to perform
            //       extra validation.

            // let pubkey = payment.extra.get("pubkey").expect("pubkey missing").to_string();
            // let content_id = payment.extra.get("content_id").expect("content_id missing").to_string();

            // pg_conn.execute("
            //     UPDATE access_table
            //     SET (pubkey, content_id, paid)
            //     VALUES ($1, $2, true)
            //     ON CONFLICT (pubkey, content_id)
            //     DO NOTHING;
            // ", &[&pubkey, &content_id]).await?;

            Ok(())
        },

        WebhookPayload::Unsupported => {
            debug!("Unsupported Event");
            Ok(())
        },
    }
}

#[post("/lnbits/webhook")]
pub async fn lnbits_webhook_handler(payload: web::Json<WebhookPayload>, app_data: web::Data<AppData>) -> impl Responder {

    dbg!(&payload);

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

    println!("Running LNbits Webhook Server on {host}:{port}");

    HttpServer::new(move || {

        let logger = Logger::default();

        let app_data = AppData {
            pg_pool: pg_pool.clone()
        };

        App::new()
            .wrap(logger)
            .app_data(web::Data::new(app_data))
            .service(routes::health_handler)
            .service(lnbits_webhook_handler)
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
    async fn test_lnbits_webhook_post_valid() {
        env::set_var("POSTGRES_ADDRESS", "postgresql://postgres:postgres@localhost:5433/postgres");

        let pg_address: String = std::env::var("POSTGRES_ADDRESS").expect("POSTGRES_ADDRESS must be set.");
        let pg_pool = pg_pool_from_url(&pg_address).unwrap();

        let app_data = AppData {
            pg_pool: pg_pool.clone()
        };

        let app = test::init_service(App::new()
                                     .app_data(web::Data::new(app_data))
                                     .service(super::lnbits_webhook_handler)).await;

        let req = test::TestRequest::default()
            .uri("/lnbits/webhook")
            .method(actix_http::Method::POST)
            .insert_header(ContentType::json())
            .set_payload(r#"{
    "checking_id": "5d7ee3af76e25677a02ecc132e0e527eab8ffc92b0af2ae5cf16aae7706ecba2",
    "pending": true,
    "amount": 10000,
    "fee": 0,
    "memo": "test webhook",
    "time": 1682947081,
    "bolt11": "lnbc100n1pjyl0qfsp533xd0y5zfakhm0ytyauhlgd0jvh0c03ucw7c7eeynaljgzh6yz9spp5t4lw8tmkuft80gpwesfjurjj064cllyjkzhj4ew0z64wwurwew3qdq5w3jhxapqwajky6r0da4sxqzjccqpjrzjqwz34f2ec60uwx0cfhmvfq9lw4j52ct98jr4p5nqwqluynewq7qkszl3wgqq9jqqqqqqqqqqqqqqqqcqjq9qyysgq70qtyljrcp64m7q8lfzezxp2zfasun9flx7mg6aej262gqxsrw94uj0w2y5664ymkapuwrv0gmdzctrjfx0j3xu9qjyeeze5yw0jkhcpqwasks",
    "preimage": "0000000000000000000000000000000000000000000000000000000000000000",
    "payment_hash": "5d7ee3af76e25677a02ecc132e0e527eab8ffc92b0af2ae5cf16aae7706ecba2",
    "expiry": 1682947681,
    "extra":
    {},
    "wallet_id": "b59d05b23e184fa69a13a68c29d62df3",
    "webhook": "https://webhook.site/d7f622fa-616f-4af7-a7ed-0a52aa504ef4",
    "webhook_status": null
}"#)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_lnbits_webhook_post_invalid() {
        env::set_var("POSTGRES_ADDRESS", "postgresql://postgres:postgres@localhost:5433/postgres");

        let pg_address: String = std::env::var("POSTGRES_ADDRESS").expect("POSTGRES_ADDRESS must be set.");
        let pg_pool = pg_pool_from_url(&pg_address).unwrap();

        let app_data = AppData {
            pg_pool: pg_pool.clone()
        };

        let app = test::init_service(App::new()
                                     .app_data(web::Data::new(app_data))
                                     .service(super::lnbits_webhook_handler)).await;

        let req = test::TestRequest::default()
            .uri("/lnbits/webhook")
            .method(actix_http::Method::POST)
            .insert_header(ContentType::json())
            .set_payload(r#"{"BAD": "JSON"}"#)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}
