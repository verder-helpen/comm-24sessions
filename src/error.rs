use crate::jwt::JwtError;
use rocket::{
    http::{ContentType, Status},
    Response,
};
use rocket_contrib::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    #[error("Bad Request: {0}")]
    BadRequest(&'static str),
    #[error("JWE Error: {0}")]
    JWE(#[from] JwtError),
    #[error("Postgres Error: {0}")]
    Postgres(#[from] postgres::Error),
    #[error("Reqwest Error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Parse Error: {0}")]
    Parse(#[from] strum::ParseError),
}

impl<'r, 'o: 'r> rocket::response::Responder<'r, 'o> for Error {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        use Error::*;
        let (body, status) = match &self {
            NotFound => (json!({"error": "NotFound"}), Status::NotFound),
            BadRequest(m) => (
                json!({"error": "BadRequest", "detail": m}),
                Status::BadRequest,
            ),
            JWE(e) => (
                json!({"error": "BadRequest", "detail": format!("{}", e)}),
                Status::BadRequest,
            ),
            _ => return rocket::response::Debug::from(self).respond_to(request),
        };
        Ok(Response::build_from(body.respond_to(request).unwrap())
            .status(status)
            .header(ContentType::JSON)
            .finalize())
    }
}

impl From<id_contact_jwt::Error> for Error {
    fn from(e: id_contact_jwt::Error) -> Self {
        Error::JWE(JwtError::JWE(e))
    }
}
