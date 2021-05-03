use id_contact_proto::AuthResult;

use crate::{error::Error, types::GuestToken, SessionDBConn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Session {
    guest_token: GuestToken,
    auth_result: Option<AuthResult>,
    attr_id: String,
}

impl Session {
    pub fn new(guest_token: GuestToken, attr_id: String) -> Self {
        Self {
            attr_id,
            guest_token,
            auth_result: None,
        }
    }

    pub async fn persist(&self, db: &SessionDBConn) -> Result<(), Error> {
        let this = self.clone();
        db.run(move |c| {
            c.execute(
                "INSERT INTO session (
                session_id,
                room_id,
                domain,
                redirect_url,
                purpose,
                name,
                instance,
                attr_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);",
                &[
                    &this.guest_token.id,
                    &this.guest_token.room_id,
                    &this.guest_token.domain.to_string(),
                    &this.guest_token.redirect_url,
                    &this.guest_token.purpose,
                    &this.guest_token.name,
                    &this.guest_token.instance,
                    &this.attr_id
                ],
            )
        })
        .await?;

        Ok(())
    }

    pub async fn register_auth_result(
        attr_id: String,
        auth_result: String,
        db: &SessionDBConn,
    ) -> Result<(), Error> {
        let n = db
            .run(move |c| {
                c.execute(
                    "UPDATE session 
                    SET auth_result = $1
                    WHERE auth_result IS NULL 
                    AND attr_id = $2;",
                    &[&auth_result, &attr_id],
                )
            })
            .await?;

        if n == 1 {
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }

    pub async fn find_by_room_id(room_id: String) -> Result<Vec<Self>, Error> {
        todo!();
    }
}
