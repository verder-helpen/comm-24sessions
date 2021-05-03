use josekit::{
    jwe::JweDecrypter,
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
pub fn decrypt_jwe(
    jwe: &str,
    validator: &dyn JwsVerifier,
    decrypter: &dyn JweDecrypter,
) -> Result<JwtPayload, JwtError> {
    let decoded_jwe = jwt::decode_with_decrypter(jwe, decrypter)?.0;
    let jws = claim_as_str(&decoded_jwe, "njwt")?;
    let decoded_jws = jwt::decode_with_verifier(jws, validator)?.0;
    let mut validator = JwtPayloadValidator::new();
    validator.set_base_time(std::time::SystemTime::now());
    validator.validate(&decoded_jws)?;
    Ok(decoded_jws)
}

pub fn claim_as_str<'a>(payload: &'a JwtPayload, key: &'static str) -> Result<&'a str, JwtError> {
    payload
        .claim(key)
        .ok_or(JwtError::InvalidStructure(key))?
        .as_str()
        .ok_or(JwtError::InvalidFormat(key))
}
pub trait FromJwt: Sized + DeserializeOwned {
    fn from_jwt(
        jwt: &str,
        validator: &dyn JwsVerifier,
        decrypter: &dyn JweDecrypter,
    ) -> Result<Self, JwtError> {
        todo!("Validate that this works");
        let payload = decrypt_jwe(jwt, validator, decrypter)?;
        // Issued at
        let _iat = claim_as_str(&payload, "iat")?;
        // Expiration
        let _exp = claim_as_str(&payload, "exp")?;
        let _rec = claim_as_str(&payload, "rec")?;

        // TODO verify iat, exp, rec

        let payload = claim_as_str(&payload, "payload")?;
        let request = serde_json::from_str(payload)?;

        Ok(request)
    }
}
