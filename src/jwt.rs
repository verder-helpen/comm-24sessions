use josekit::{
    jws::JwsVerifier,
    jwt::{self, JwtPayload, JwtPayloadValidator},
};
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Invalid Structure for key {0}")]
    InvalidStructure(&'static str),
    #[error("Invalid Data type for key {0}")]
    InvalidFormat(&'static str),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("24 Sessions JWT error: {0}")]
    JWT(#[from] josekit::JoseError),
    #[error("ID Contact JWE error: {0}")]
    JWE(#[from] id_contact_jwt::Error),
}

/// Decrypt and verify a given jwe to extract the contained attributes.
pub fn validate_jws(jws: &str, validator: &dyn JwsVerifier) -> Result<JwtPayload, JwtError> {
    let decoded_jws = jwt::decode_with_verifier(jws, validator)?.0;
    let mut validator = JwtPayloadValidator::new();
    validator.set_base_time(std::time::SystemTime::now());
    validator.validate(&decoded_jws)?;
    Ok(decoded_jws)
}

// Try to get a claim as &str
pub fn claim_as_str<'a>(payload: &'a JwtPayload, key: &'static str) -> Result<&'a str, JwtError> {
    payload
        .claim(key)
        .ok_or(JwtError::InvalidStructure(key))?
        .as_str()
        .ok_or(JwtError::InvalidFormat(key))
}

pub trait FromJws: Sized + DeserializeOwned {
    fn from_jws(jwt: &str, validator: &dyn JwsVerifier) -> Result<Self, JwtError> {
        let payload = validate_jws(jwt, validator)?;
        let payload = claim_as_str(&payload, "payload")?;
        let payload = serde_json::from_str(payload)?;
        Ok(payload)
    }
}


pub fn hmac256_validator(key: &str) {
    
}