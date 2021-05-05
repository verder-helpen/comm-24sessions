use core::str;
use std::collections::HashMap;

use id_contact_proto::AuthResult;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, ToString};

use crate::jwt::FromJws;

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
    #[serde(rename = "redirectURL")]
    pub redirect_url: String,
    #[serde(rename = "authPurpuse")]
    pub purpose: String,
    pub name: String,
    #[serde(rename = "roomId")]
    pub room_id: String,
    pub instance: String,
}

impl FromJws for GuestToken {}

impl FromJws for HostToken {}

pub type AuthResultSet = HashMap<String, Option<AuthResult>>;