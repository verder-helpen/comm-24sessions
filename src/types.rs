use core::str;
use std::collections::HashMap;

use id_contact_proto::AuthResult;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, ToString};

use crate::jwt::From24SessionsJwt;

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
    #[serde(rename = "authPurpose")]
    pub purpose: Option<String>,
    pub name: String,
    #[serde(rename = "roomId")]
    pub room_id: String,
    pub instance: String,
}

impl From24SessionsJwt for GuestToken {}

impl From24SessionsJwt for HostToken {}

pub type AuthResultSet = HashMap<String, Option<AuthResult>>;
