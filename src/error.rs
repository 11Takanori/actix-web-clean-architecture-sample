use actix_web::{error, http::StatusCode, HttpResponse};
use failure::Fail;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "An internal error occurred. Please try again later.")]
    InternalServerError,
    #[fail(display = "Not Found")]
    NotFound,
}

impl error::ResponseError for Error {
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

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        if let Some(status) = error.status() {
            if status == 404 {
                return Error::NotFound;
            }
        }

        Error::InternalServerError
    }
}
