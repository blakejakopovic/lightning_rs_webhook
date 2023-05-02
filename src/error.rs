use actix_web::{
    error,
    http::{header::ContentType, StatusCode}, HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServiceError {
    #[display(fmt = "Internal Error")]
    InternalError,

    #[display(fmt = "Bad Request")]
    BadClientData,

    #[display(fmt = "Timeout")]
    Timeout,
}

impl error::ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadClientData => StatusCode::BAD_REQUEST,
            ServiceError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
