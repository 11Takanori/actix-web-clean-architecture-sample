use actix_web::{http::StatusCode, HttpResponse};
use failure::Fail;

// FIXME: error handling

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "An internal error occurred. Please try again later.")]
    InternalServerError,
    #[fail(display = "Not Found")]
    NotFound,
}

impl actix_web::error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::NotFound => HttpResponse::NotFound().json("Not Found"),
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        }
    }
}

impl From<awc::error::SendRequestError> for Error {
    fn from(_error: awc::error::SendRequestError) -> Self {
        Error::InternalServerError
    }
}

impl From<awc::error::JsonPayloadError> for Error {
    fn from(_error: awc::error::JsonPayloadError) -> Self {
        Error::InternalServerError
    }
}
