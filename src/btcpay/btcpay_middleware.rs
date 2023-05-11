use anyhow::Result;
use actix_http::h1;
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpResponse, body::EitherBody,
};
use crate::btcpay::verify_signature;
use futures_util::future::LocalBoxFuture;
use lazy_static::lazy_static;
use std::{
    future::{ready, Ready},
    rc::Rc,
};


lazy_static! {
    static ref BTCPAY_WEBHOOK_SECRET: String = std::env::var("BTCPAY_WEBHOOK_SECRET").expect("BTCPAY_WEBHOOK_SECRET must be set");
}

pub const BTCPAY_SIG_HEADER: &str = "BTCPay-Sig";

pub fn bytes_to_payload(buf: web::Bytes) -> dev::Payload {
    let (_, mut pl) = h1::Payload::create(true);
    pl.unread_data(buf);
    dev::Payload::from(pl)
}

pub struct BTCPayHeaderVerify;

impl<S: 'static, B> Transform<S, ServiceRequest> for BTCPayHeaderVerify
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    // type Response = ServiceResponse<B>;
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = BTCPayVerifyMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(BTCPayVerifyMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct BTCPayVerifyMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for BTCPayVerifyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    // type Response = ServiceResponse<B>;
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {

            let headers = req.headers().clone();

            let btcpay_sig_header = if let Some(header_value) = headers.get(BTCPAY_SIG_HEADER) {
                match header_value.to_str() {
                    Ok(value) => value,
                    // Invalid header value
                    Err(_) => return Ok(return_unauthorized(req)),
                }
            } else {
                // Missing header
                return Ok(return_unauthorized(req));
            };

            debug!("Header: {BTCPAY_SIG_HEADER}: {btcpay_sig_header}");

            // Borrow request body bytes
            let body_bytes = match req.extract::<web::Bytes>().await {
                Ok(body_bytes) => body_bytes,
                Err(_) => return Ok(return_unauthorized(req)),
            };

            // Re-insert body bytes back into request
            req.set_payload(bytes_to_payload(body_bytes.clone()));

            // Convery body from bytes to utf8 string
            let body_str = match std::str::from_utf8(&body_bytes) {
                Ok(body_str) => body_str,
                Err(_) => return Ok(return_unauthorized(req)),
            };

            debug!("Body: {body_str}");

            if let false = verify_signature(body_str, &BTCPAY_WEBHOOK_SECRET, &btcpay_sig_header) {
                error!("Bad signature. Check webhook secret, or unauthorised request.");
                return Ok(return_unauthorized(req))
            }

            let res = svc.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}

fn return_unauthorized<L>(req: ServiceRequest) -> ServiceResponse<EitherBody<L>> {
    let (request, _pl) = req.into_parts();
    let response = HttpResponse::Unauthorized().finish().map_into_right_body::<L>();
    ServiceResponse::new(request, response)
}
