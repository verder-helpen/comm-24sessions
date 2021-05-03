use core::str;

use serde::{Deserialize, Serialize};
use strum_macros::ToString;

use crate::jwt::FromJwt;

#[derive(Deserialize, Debug, Serialize, ToString, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum SessionDomain {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "guest")]
    Guest,
}

#[derive(Deserialize, Debug)]
pub struct HostToken {
    id: String,
    domain: SessionDomain,
    #[serde(rename = "roomId")]
    room_id: String,
    instance: String,
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

impl FromJwt for GuestToken {}

impl FromJwt for HostToken {}
