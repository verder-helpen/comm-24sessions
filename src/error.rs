use crate::jwt::JwtError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not found")]
    NotFound,
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
        let debug_error = rocket::response::Debug::from(self);
        debug_error.respond_to(request)
    }
}

impl From<id_contact_jwt::Error> for Error {
    fn from(e: id_contact_jwt::Error) -> Self {
        Error::JWE(JwtError::JWE(e))
    }
}
