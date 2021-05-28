use crate::error::Error;

use id_contact_jwt::{EncryptionKeyConfig, SignKeyConfig};
use josekit::{
    jwe::JweDecrypter,
    jws::{alg::hmac::HmacJwsAlgorithm, JwsSigner, JwsVerifier},
};
use serde::Deserialize;

use std::convert::TryFrom;

#[derive(Deserialize, Debug)]
struct RawConfig {
    internal_url: String,
    external_url: String,
    core_url: String,
    widget_url: String,
    display_name: String,

    decryption_privkey: EncryptionKeyConfig,
    signature_pubkey: SignKeyConfig,
    widget_signature_privkey: SignKeyConfig,
    guest_signature_secret: String,
    host_signature_secret: String,
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "RawConfig")]
pub struct Config {
    internal_url: String,
    external_url: String,
    core_url: String,
    widget_url: String,
    display_name: String,

    decrypter: Box<dyn JweDecrypter>,
    validator: Box<dyn JwsVerifier>,
    widget_signer: Box<dyn JwsSigner>,
    guest_validator: Box<dyn JwsVerifier>,
    host_validator: Box<dyn JwsVerifier>,
}

// This tryfrom can be removed once try_from for fields lands in serde
impl TryFrom<RawConfig> for Config {
    type Error = Error;
    fn try_from(config: RawConfig) -> Result<Config, Error> {
        let guest_validator = HmacJwsAlgorithm::Hs256
            .verifier_from_bytes(config.guest_signature_secret)
            .unwrap();
        let host_validator = HmacJwsAlgorithm::Hs256
            .verifier_from_bytes(config.host_signature_secret)
            .unwrap();

        Ok(Config {
            internal_url: config.internal_url,
            external_url: config.external_url,
            core_url: config.core_url,
            widget_url: config.widget_url,
            display_name: config.display_name,

            decrypter: Box::<dyn JweDecrypter>::try_from(config.decryption_privkey)?,
            validator: Box::<dyn JwsVerifier>::try_from(config.signature_pubkey)?,
            widget_signer: Box::<dyn JwsSigner>::try_from(config.widget_signature_privkey)?,
            guest_validator: Box::new(guest_validator),
            host_validator: Box::new(host_validator),
        })
    }
}

impl Config {
    pub fn decrypter(&self) -> &dyn JweDecrypter {
        self.decrypter.as_ref()
    }

    pub fn validator(&self) -> &dyn JwsVerifier {
        self.validator.as_ref()
    }

    pub fn widget_signer(&self) -> &dyn JwsSigner {
        self.widget_signer.as_ref()
    }

    pub fn guest_validator(&self) -> &dyn JwsVerifier {
        self.guest_validator.as_ref()
    }

    pub fn host_validator(&self) -> &dyn JwsVerifier {
        self.host_validator.as_ref()
    }

    pub fn internal_url(&self) -> &str {
        &self.internal_url
    }

    pub fn external_url(&self) -> &str {
        &self.external_url
    }

    pub fn core_url(&self) -> &str {
        &self.core_url
    }

    pub fn widget_url(&self) -> &str {
        &self.widget_url
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}
