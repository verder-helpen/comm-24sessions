use id_contact_comm_common::{
    auth::{check_token, render_login, render_unauthorized, TokenCookie},
    config::Config,
    credentials::{get_credentials_for_host, render_credentials},
    error::Error,
    jwt::sign_auth_select_params,
    session::{Session, SessionDBConn},
    templates::{RenderType, RenderedContent},
    types::{AuthSelectParams, FromPlatformJwt, GuestToken, StartRequest},
    util::random_string,
};
use id_contact_proto::{ClientUrlResponse, StartRequestAuthOnly};
use rocket::{get, launch, post, response::Redirect, routes, serde::json::Json, State};

#[get("/init/<guest_token>")]
async fn init(guest_token: String, config: &State<Config>) -> Result<Redirect, Error> {
    let GuestToken { purpose, .. } = GuestToken::from_platform_jwt(
        &guest_token,
        config.auth_during_comm_config().guest_verifier(),
    )?;

    let auth_select_params = AuthSelectParams {
        purpose,
        start_url: format!("{}/start/{}", config.external_url(), guest_token),
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

    let session = Session::new(guest_token, attr_id);

    session.persist(&db).await?;

    let start_request = StartRequestAuthOnly {
        purpose: session.guest_token.purpose,
        auth_method,
        comm_url,
        attr_url: Some(attr_url),
    };

    let start_request = id_contact_comm_common::jwt::sign_start_auth_request(
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
) -> Result<(), Error> {
    id_contact_jwt::decrypt_and_verify_auth_result(
        &auth_result,
        config.verifier(),
        config.decrypter(),
    )?;
    Session::register_auth_result(attr_id, auth_result, &db).await
}

#[get("/session_info/<host_token>")]
async fn session_info(
    host_token: String,
    config: &State<Config>,
    db: SessionDBConn,
    token: TokenCookie,
) -> Result<RenderedContent, Error> {
    if check_token(token, config).await? {
        let credentials = get_credentials_for_host(host_token, config, db)
            .await
            .unwrap_or_else(|_| Vec::new());

        return render_credentials(credentials, RenderType::Html);
    }

    render_unauthorized(config, RenderType::Html)
}

#[allow(unused_variables)]
#[get("/session_info/<host_token>", rank = 2)]
async fn session_info_anon(
    host_token: String,
    config: &State<Config>,
) -> Result<RenderedContent, Error> {
    render_login(config, RenderType::Html)
}

#[get("/clean_db")]
async fn clean_db(db: SessionDBConn) -> Result<(), Error> {
    id_contact_comm_common::session::clean_db(&db).await
}

#[launch]
fn rocket() -> _ {
    id_contact_sentry::SentryLogger::init();
    let mut base = rocket::build()
        .mount(
            "/",
            routes![
                init,
                start,
                auth_result,
                session_info,
                session_info_anon,
                clean_db,
            ],
        )
        .attach(SessionDBConn::fairing());

    let config = base.figment().extract::<Config>().unwrap_or_else(|_| {
        // Drop error value, as it could contain secrets
        panic!("Failure to parse configuration")
    });

    // attach Auth provider fairing
    if let Some(auth_provider) = config.auth_provider() {
        base = base.attach(auth_provider.fairing());
    }

    if let Some(sentry_dsn) = config.sentry_dsn() {
        base = base.attach(id_contact_sentry::SentryFairing::new(
            sentry_dsn,
            "comm-24sessions",
        ));
    }

    base.manage(config)
}
