use rocket::http::Status;
use rocket::response::content::RawJavaScript;
use rocket::response::stream::{Event, EventStream};
use rocket::response::{content::RawHtml, status};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{get, post, response::Redirect, routes, serde::json::Json, Shutdown, State};
use verder_helpen_comm_common::{
    auth::{render_login, render_not_found, Authorized},
    config::Config,
    credentials::{get_credentials_for_host, render_credentials},
    error::Error,
    jwt::sign_auth_select_params,
    session::{periodic_cleanup, Session, SessionDBConn},
    templates::{RenderType, RenderedContent},
    translations::Translations,
    types::{AuthSelectParams, FromPlatformJwt, GuestToken, HostToken, StartRequest},
    util::random_string,
};
use verder_helpen_proto::{ClientUrlResponse, StartRequestAuthOnly};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AttributesReceivedEvent {
    pub attr_id: String,
}

#[get("/init/<guest_token>")]
async fn init(guest_token: String, config: &State<Config>) -> Result<Redirect, Error> {
    let GuestToken {
        purpose,
        redirect_url,
        ..
    } = GuestToken::from_platform_jwt(
        &guest_token,
        config.auth_during_comm_config().guest_verifier(),
    )?;

    let auth_select_params = AuthSelectParams {
        purpose,
        start_url: format!("{}/start/{}", config.external_guest_url(), guest_token),
        cancel_url: redirect_url,
        display_name: config.auth_during_comm_config().display_name().to_owned(),
    };

    let auth_select_params = sign_auth_select_params(
        auth_select_params,
        config.auth_during_comm_config().widget_signer(),
    )?;
    let uri = format!(
        "{}{}",
        config.auth_during_comm_config().widget_url(),
        auth_select_params
    );
    Ok(Redirect::to(uri))
}

#[post("/start/<guest_token>", data = "<start_request>")]
async fn start(
    guest_token: String,
    start_request: String,
    config: &State<Config>,
    db: SessionDBConn,
) -> Result<Json<ClientUrlResponse>, Error> {
    let guest_token = GuestToken::from_platform_jwt(
        &guest_token,
        config.auth_during_comm_config().guest_verifier(),
    )?;
    let StartRequest {
        purpose,
        auth_method,
    } = serde_json::from_str(&start_request)?;

    if purpose != guest_token.purpose {
        return Err(Error::BadRequest(
            "Purpose from start request does not match guest token purpose.",
        ));
    }

    let attr_id = random_string(64);
    let comm_url = guest_token.redirect_url.clone();
    let attr_url = format!("{}/auth_result/{}", config.internal_url(), attr_id);
    let purpose = guest_token.purpose.clone();
    if !Session::restart_auth(guest_token.clone(), attr_id.clone(), &db).await? {
        let session = Session::new(guest_token, attr_id);

        session.persist(&db).await?;
    }

    let start_request = StartRequestAuthOnly {
        purpose,
        auth_method,
        comm_url,
        attr_url: Some(attr_url),
    };

    let start_request = verder_helpen_comm_common::jwt::sign_start_auth_request(
        start_request,
        config.auth_during_comm_config().start_auth_key_id(),
        config.auth_during_comm_config().start_auth_signer(),
    )?;

    let client = reqwest::Client::new();
    let client_url_response = client
        .post(format!(
            "{}/start",
            config.auth_during_comm_config().core_url()
        ))
        .header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        )
        .header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/jwt"),
        )
        .body(start_request)
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
    queue: &State<Sender<AttributesReceivedEvent>>,
) -> Result<(), Error> {
    verder_helpen_jwt::decrypt_and_verify_auth_result(
        &auth_result,
        config.verifier(),
        config.decrypter(),
    )?;
    let response = Session::register_auth_result(attr_id.clone(), auth_result, &db).await;

    // may fail when there are no subscribers
    let _ = queue.send(AttributesReceivedEvent { attr_id });

    response
}

#[get("/live/session_info/<host_token>")]
async fn live_session_info(
    queue: &State<Sender<AttributesReceivedEvent>>,
    mut end: Shutdown,
    host_token: String,
    config: &State<Config>,
    sse_config: &State<SseConfig>,
    db: SessionDBConn,
    authorized: Authorized,
) -> EventStream![] {
    let mut rx = queue.subscribe();

    let host_token = HostToken::from_platform_jwt(
        &host_token,
        config.auth_during_comm_config().host_verifier(),
    )
    .unwrap();

    let timeout = sse_config.sse_timeout;

    EventStream! {
        if authorized.into() {
            yield Event::data("start");

            let sleeper = rocket::tokio::time::sleep(timeout);
            rocket::tokio::pin!(sleeper);

            loop {
                select! {
                    msg = rx.recv() => match msg {
                        Ok(msg) => {
                            // fetch all attribute ids related to the provided host token
                            if let Ok(sessions) = Session::find_by_room_id(
                                host_token.room_id.clone(),
                                &db
                            ).await {
                                let attr_ids: Vec<String> = sessions
                                    .iter()
                                    .map(|session: &Session| session.attr_id.clone())
                                    .collect();

                                if attr_ids.contains(&msg.attr_id) {
                                    yield Event::data("update");
                                }
                            };
                        },
                        Err(RecvError::Closed) => break,
                        Err(RecvError::Lagged(_)) => continue,
                    },
                    _ = &mut sleeper => break,
                    _ = &mut end => break,
                };
            }
        } else {
            yield Event::data("forbidden");
        }
    }
}

#[get("/session_info/<host_token>")]
async fn session_info(
    host_token: String,
    config: &State<Config>,
    db: SessionDBConn,
    authorized: Authorized,
    translations: Translations,
) -> Result<status::Custom<RenderedContent>, Error> {
    if authorized.into() {
        let credentials = get_credentials_for_host(host_token, config, &db)
            .await
            .unwrap_or_else(|_| Vec::new());

        // return 404 when no credentials are found
        if credentials.is_empty() {
            return Ok(status::Custom(
                Status::NotFound,
                render_not_found(config, RenderType::Html, translations)?,
            ));
        }

        return Ok(status::Custom(
            Status::Ok,
            render_credentials(credentials, RenderType::Html, translations)?,
        ));
    }

    Ok(status::Custom(
        Status::Unauthorized,
        render_login(config, RenderType::Html, translations)?,
    ))
}

#[get("/clean_db")]
async fn clean_db(db: SessionDBConn) -> Result<(), Error> {
    verder_helpen_comm_common::session::clean_db(&db).await
}

#[get("/<_token>")]
async fn attribute_ui(_token: String) -> RawHtml<&'static str> {
    RawHtml(include_str!("../attribute-ui/index.html"))
}

#[get("/attribute.js")]
async fn attribute_js() -> RawJavaScript<&'static str> {
    RawJavaScript(include_str!("../attribute-ui/attribute.js"))
}

#[derive(Debug, Deserialize)]
struct SseConfig {
    sse_timeout: std::time::Duration,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    verder_helpen_sentry::SentryLogger::init();
    let mut base = rocket::build()
        .manage(channel::<AttributesReceivedEvent>(1024).0)
        .mount("/internal", routes![auth_result, clean_db,])
        .mount("/guest", routes![init, start,])
        .mount(
            "/host",
            routes![live_session_info, session_info, attribute_ui, attribute_js,],
        )
        .attach(SessionDBConn::fairing());

    let config = base.figment().extract::<Config>().unwrap_or_else(|_| {
        // Drop error value, as it could contain secrets
        panic!("Failure to parse configuration")
    });

    let sse_config = base.figment().extract::<SseConfig>().unwrap_or_else(|_| {
        // Drop error value, as it could contain secrets
        panic!("Failure to parse configuration")
    });

    // attach Auth provider fairing
    if let Some(auth_provider) = config.auth_provider() {
        base = base.attach(auth_provider.fairing());
    }

    if let Some(sentry_dsn) = config.sentry_dsn() {
        base = base.attach(verder_helpen_sentry::SentryFairing::new(
            sentry_dsn,
            "comm-24sessions",
        ));
    }

    let base = base
        .manage(config)
        .manage(sse_config)
        .ignite()
        .await
        .expect("Failed to ignite");

    let connection = SessionDBConn::get_one(&base)
        .await
        .expect("Failed to fetch database connection for periodic cleanup");
    rocket::tokio::spawn(async move {
        periodic_cleanup(&connection, None)
            .await
            .expect("Failed cleanup");
    });

    // manually ignoring `unused_must_use` for Rocket version 0.5.0-rc.2
    let _ = base.launch().await?;
    Ok(())
}
