use jwt::From24SessionsJwt;
use rocket::{fairing::AdHoc, get, launch, post, response::Redirect, routes, State};
use rocket_contrib::{database, databases::postgres, json::Json};
use types::{AuthResultSet, GuestAuthResult, GuestToken, HostToken};

use crate::{
    config::Config,
    error::Error,
    session::Session,
    types::{StartRequest, WidgetRedirectParams},
    util::random_string,
};
use id_contact_proto::{ClientUrlResponse, StartRequestAuthOnly};

mod config;
mod error;
mod jwt;
mod session;
mod types;
mod util;

#[database("session")]
pub struct SessionDBConn(postgres::Client);

#[get("/init/<purpose>/<guest_token>")]
async fn init(
    purpose: String,
    guest_token: String,
    config: &State<Config>,
) -> Result<Redirect, Error> {
    let _ = GuestToken::from_24sessions_jwt(&guest_token, config.guest_validator())?;

    let redirect_params = WidgetRedirectParams {
        purpose,
        start_url: format!("{}/start/{}", config.external_url(), guest_token),
        display_name: config.display_name().to_owned(),
    };
    let redirect_params = redirect_params.to_jws(config.widget_signer())?;
    let uri = format!("{}?{}", config.widget_url(), redirect_params);

    Ok(Redirect::to(uri))
}

#[post("/start/<guest_token>", data = "<start_request>")]
async fn start(
    guest_token: String,
    start_request: String,
    config: &State<Config>,
    db: SessionDBConn,
) -> Result<Json<ClientUrlResponse>, Error> {
    let guest_token = GuestToken::from_24sessions_jwt(&guest_token, config.guest_validator())?;
    let StartRequest {
        purpose,
        auth_method,
    } = serde_json::from_str(&start_request)?;
    let attr_id = random_string(64);
    let comm_url = guest_token.redirect_url.clone();
    let attr_url = format!("{}/auth_result/{}", config.internal_url(), attr_id);

    let session = Session::new(guest_token, attr_id, purpose.clone());

    session.persist(&db).await?;

    let start_request = StartRequestAuthOnly {
        purpose,
        auth_method,
        comm_url,
        attr_url: Some(attr_url),
    };

    let client = reqwest::Client::new();
    let client_url_response = client
        .post(format!("{}/start", config.core_url()))
        .json(&start_request)
        .send()
        .await?
        .text()
        .await?;

    let client_url_response = serde_json::from_str::<ClientUrlResponse>(&client_url_response)?;
    Ok(Json(client_url_response))
}

#[post("/auth_result/<attr_id>", data = "<auth_result>")]
async fn auth_result(
    attr_id: String,
    auth_result: String,
    config: &State<Config>,
    db: SessionDBConn,
) -> Result<(), Error> {
    id_contact_jwt::decrypt_and_verify_auth_result(
        &auth_result,
        config.validator(),
        config.decrypter(),
    )?;
    Session::register_auth_result(attr_id, auth_result, &db).await
}

#[get("/session_info/<host_token>")]
async fn session_info(
    host_token: String,
    config: &State<Config>,
    db: SessionDBConn,
) -> Result<Json<AuthResultSet>, Error> {
    let host_token = HostToken::from_24sessions_jwt(&host_token, config.host_validator())?;
    let sessions = match Session::find_by_room_id(host_token.room_id, &db).await {
        Ok(s) => s,
        // Return empty object if no session was found
        Err(Error::NotFound) => return Ok(Json(AuthResultSet::new())),
        e => e?,
    };

    let auth_results: AuthResultSet = sessions
        .into_iter()
        .map(|s| {
            (
                s.guest_token.id,
                GuestAuthResult {
                    name: s.guest_token.name,
                    attributes: s.auth_result
                        .map(|r| {
                                id_contact_jwt::dangerous_decrypt_auth_result_without_verifying_expiration(
                                    &r,
                                    config.validator(),
                                    config.decrypter(),
                            )
                            .map(|r| r.attributes)
                            .ok()
                    })
                    .flatten()
                    .flatten()
            },
            )
        })
        .filter(|(_, g)| g.attributes.is_some())
        .collect();
    Ok(Json(auth_results))
}

#[get("/clean_db")]
async fn clean_db(db: SessionDBConn) -> Result<(), Error> {
    session::clean_db(&db).await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![init, start, auth_result, session_info, clean_db,],
        )
        .attach(SessionDBConn::fairing())
        .attach(AdHoc::config::<Config>())
}
