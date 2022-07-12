use serde::Serialize;
// errors
use surf::{Response, StatusCode};

#[derive(Debug, Serialize)]
pub enum Error {
    NOTFOUND,
    UNKNOWN,
    FORBIDDEN,
    UNAUTHORIZED,
    BADREQUEST,
    CONFLICT,
    UNPROCESSABLEENTITY,
    INTERNALSERVERERROR,
    PRECODINTIONFAILED,
    BADGATEWAY,
    LISTSTREAMS,
}

impl Error {
    pub fn from_response(response: &Response) -> Error {
        match response.status() {
            StatusCode::NotFound => Error::NOTFOUND,
            StatusCode::Forbidden => Error::FORBIDDEN,
            StatusCode::Unauthorized => Error::UNAUTHORIZED,
            StatusCode::BadRequest => Error::BADREQUEST,
            StatusCode::Conflict => Error::CONFLICT,
            StatusCode::UnprocessableEntity => Error::UNPROCESSABLEENTITY,
            StatusCode::InternalServerError => Error::INTERNALSERVERERROR,
            StatusCode::PreconditionFailed => Error::PRECODINTIONFAILED,
            StatusCode::BadGateway => Error::BADGATEWAY,
            _ => Error::UNKNOWN,
        }
    }
}
