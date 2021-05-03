use crate::error::Error;

use id_contact_jwt::{EncryptionKeyConfig, SignKeyConfig};
use josekit::{jwe::JweDecrypter, jws::JwsVerifier};
use serde::Deserialize;

use std::convert::TryFrom;

#[derive(Deserialize, Debug)]
struct RawConfig {
    internal_url: String,
    external_url: String,
    core_url: String,

    decryption_privkey: EncryptionKeyConfig,
    signature_pubkey: SignKeyConfig,
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "RawConfig")]
pub struct Config {
    internal_url: String,
    external_url: String,
    core_url: String,

    decrypter: Box<dyn JweDecrypter>,
    validator: Box<dyn JwsVerifier>,
}

// This tryfrom can be removed once try_from for fields lands in serde
impl TryFrom<RawConfig> for Config {
    type Error = Error;
    fn try_from(config: RawConfig) -> Result<Config, Error> {
        Ok(Config {
            internal_url: config.internal_url,
            external_url: config.external_url,
            core_url: config.core_url,

            decrypter: Box::<dyn JweDecrypter>::try_from(config.decryption_privkey)?,
            validator: Box::<dyn JwsVerifier>::try_from(config.signature_pubkey)?,
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

    pub fn internal_url(&self) -> &str {
        &self.internal_url
    }

    pub fn external_url(&self) -> &str {
        &self.external_url
    }

    pub fn core_url(&self) -> &str {
        &self.core_url
    }
}
