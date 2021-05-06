use josekit::jws::JwsVerifier;
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Invalid Structure for key {0}")]
    InvalidStructure(&'static str),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("24 Sessions JWT error: {0}")]
    JWT(#[from] josekit::JoseError),
    #[error("ID Contact JWE error: {0}")]
    JWE(#[from] id_contact_jwt::Error),
}

pub trait From24SessionsJwt: Sized + DeserializeOwned {
    fn from_24sessions_jwt(jwt: &str, validator: &dyn JwsVerifier) -> Result<Self, JwtError> {
        let (payload, _) = josekit::jwt::decode_with_verifier(jwt, validator)?;
        let claim = payload
            .claim("payload")
            .ok_or(JwtError::InvalidStructure("payload"))?;
        let payload = serde_json::from_value(claim.clone())?;
        Ok(payload)
    }
}
