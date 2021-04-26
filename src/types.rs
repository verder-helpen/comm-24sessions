use core::str;

use rocket::request::FromRequest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct InitSessionRequest {
    meeting_id: String,
}

#[derive(Serialize, Debug)]
pub struct InitSessionResponse {

}
