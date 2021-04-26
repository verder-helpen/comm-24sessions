use rocket::{fairing::AdHoc, launch, post, routes, State};
use rocket_contrib::{database, databases::postgres, json::Json};
use types::{InitSessionRequest, InitSessionResponse};

use crate::{config::Config, error::Error};

mod comm;
mod config;
mod error;
mod types;

#[database("session")]
pub struct SessionDBConn(postgres::Client);

#[post("/init_session", data = "<request>")]
async fn init_session(
    request: Json<InitSessionRequest>,
    config: State<'_, Config>,
    db: SessionDBConn,
) -> Result<Json<InitSessionResponse>, Error> {
    dbg!(request, config);
    todo!()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![init_session,])
        .attach(SessionDBConn::fairing())
        .attach(AdHoc::config::<Config>())
}
