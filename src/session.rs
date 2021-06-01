use std::str::FromStr;

use crate::{
    error::Error,
    types::{GuestToken, SessionDomain},
    SessionDBConn,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Session {
    pub guest_token: GuestToken,
    pub auth_result: Option<String>,
    pub attr_id: String,
    pub purpose: String,
}

impl Session {
    pub fn new(guest_token: GuestToken, attr_id: String, purpose: String) -> Self {
        Self {
            attr_id,
            purpose,
            guest_token,
            auth_result: None,
        }
    }

    /// Persist a sessions. This can only be done for newly created sessions,
    /// as the session id is unique.
    pub async fn persist(&self, db: &SessionDBConn) -> Result<(), Error> {
        let this = self.clone();
        let res = db
            .run(move |c| {
                c.execute(
                    "INSERT INTO session (
                session_id,
                room_id,
                domain,
                redirect_url,
                purpose,
                name,
                instance,
                attr_id,
                auth_result
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);",
                    &[
                        &this.guest_token.id,
                        &this.guest_token.room_id,
                        &this.guest_token.domain.to_string(),
                        &this.guest_token.redirect_url,
                        &this.purpose,
                        &this.guest_token.name,
                        &this.guest_token.instance,
                        &this.attr_id,
                        &this.auth_result,
                    ],
                )
            })
            .await;

        res.map_err(|e| {
            if let Some(&postgres::error::SqlState::UNIQUE_VIOLATION) = e.code() {
                Error::BadRequest("A session with that ID already exists")
            } else {
                Error::from(e)
            }
        })?;
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

        match n {
            1 => Ok(()),
            _ => Err(Error::NotFound),
        }
    }

    pub async fn find_by_room_id(room_id: String, db: &SessionDBConn) -> Result<Vec<Self>, Error> {
        let sessions = db
            .run(move |c| -> Result<Vec<Session>, Error> {
                let rows = c.query(
                    "SELECT 
                        session_id,
                        room_id,
                        domain,
                        redirect_url,
                        purpose,
                        name,
                        instance,
                        attr_id,
                        auth_result
                    FROM session 
                    WHERE room_id = $1",
                    &[&room_id],
                )?;
                if rows.len() == 0 {
                    return Err(Error::NotFound);
                }
                rows.into_iter()
                    .map(|r| -> Result<_, Error> {
                        let domain = SessionDomain::from_str(r.get("domain"))?;
                        let guest_token = GuestToken {
                            id: r.get("session_id"),
                            room_id: r.get("room_id"),
                            domain,
                            redirect_url: r.get("redirect_url"),
                            name: r.get("name"),
                            instance: r.get("instance"),
                        };
                        Ok(Session {
                            purpose: r.get("purpose"),
                            guest_token,
                            attr_id: r.get("attr_id"),
                            auth_result: r.get("auth_result"),
                        })
                    })
                    .collect()
            })
            .await?;

        Ok(sessions)
    }
}
