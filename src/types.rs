use core::str;
use std::collections::HashMap;

use crate::jwt::{From24SessionsJwt, JwtError};
use josekit::{
    jws::{JwsHeader, JwsSigner},
    jwt::JwtPayload,
};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, ToString};

#[derive(Deserialize, Debug, Serialize, ToString, Clone, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum SessionDomain {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "guest")]
    Guest,
}

#[derive(Deserialize, Debug)]
pub struct HostToken {
    pub id: String,
    pub domain: SessionDomain,
    #[serde(rename = "roomId")]
    pub room_id: String,
    pub instance: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GuestToken {
    pub id: String,
    pub domain: SessionDomain,
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
    pub name: String,
    #[serde(rename = "roomId")]
    pub room_id: String,
    pub instance: String,
}

impl From24SessionsJwt for GuestToken {}

impl From24SessionsJwt for HostToken {}

#[derive(Serialize, Debug)]
pub struct GuestAuthResult {
    pub attributes: Option<HashMap<String, String>>,
    pub name: String,
}

pub type AuthResultSet = HashMap<String, GuestAuthResult>;

#[derive(Deserialize, Debug)]
pub struct StartRequest {
    pub purpose: String,
    pub auth_method: String,
}

#[derive(Serialize, Debug)]
pub struct WidgetRedirectParams {
    pub purpose: String,
    pub start_url: String,
    pub display_name: String,
}

impl WidgetRedirectParams {
    pub fn to_jws(&self, signer: &dyn JwsSigner) -> Result<String, JwtError> {
        let mut sig_header = JwsHeader::new();
        sig_header.set_token_type("JWT");
        let mut sig_payload = JwtPayload::new();
        sig_payload.set_subject("id-contact-widget-params");

        sig_payload.set_claim("purpose", Some(serde_json::to_value(&self.purpose)?))?;
        sig_payload.set_claim("start_url", Some(serde_json::to_value(&self.start_url)?))?;
        sig_payload.set_claim(
            "display_name",
            Some(serde_json::to_value(&self.display_name)?),
        )?;

        sig_payload.set_issued_at(&std::time::SystemTime::now());
        sig_payload.set_expires_at(
            &(std::time::SystemTime::now() + std::time::Duration::from_secs(5 * 60)),
        );

        let jws = josekit::jwt::encode_with_signer(&sig_payload, &sig_header, signer)?;

        Ok(jws)
    }
}
