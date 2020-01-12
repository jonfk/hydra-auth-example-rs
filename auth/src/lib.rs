use log::info;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use warp::{self, http::Uri, Filter};

use hydra::apis::{configuration::Configuration, AdminApi, AdminApiClient};
use std::{str::FromStr, sync::Arc};

pub mod view;

const HYDRA_ADMIN_ADDRESS: &'static str = "http://localhost:4445";

pub fn routes() -> warp::filters::BoxedFilter<(impl warp::reply::Reply,)> {
    auth_routes()
        .or(homepage())
        .with(warp::log("api_access"))
        .boxed()
}

pub fn homepage() -> warp::filters::BoxedFilter<(impl warp::reply::Reply,)> {
    warp::get()
        .map(move || {
            let home_body = r#"
<!DOCTYPE html>
<head>
    <meta charset="UTF-8">
    <title>Home</title>
</head>
<body>
<p>Welcome</p>
</body>
</html>
"#;
            warp::reply::html(home_body)
        })
        .boxed()
}

pub fn with_hydra_api() -> warp::filters::BoxedFilter<(AdminApiClient,)> {
    warp::any()
        .map(move || {
            let mut configuration = Configuration::new();
            configuration.base_path = HYDRA_ADMIN_ADDRESS.to_owned();
            AdminApiClient::new(Arc::new(configuration))
        })
        .boxed()
}

pub fn auth_routes() -> warp::filters::BoxedFilter<(impl warp::reply::Reply,)> {
    let login_routes = warp::path("login").and(login_page().or(accept_login()));
    let consent_routes = warp::path("consent").and(consent_page().or(accept_consent()));
    login_routes.or(consent_routes).boxed()
}

pub fn login_page() -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct LoginQueryParams {
        login_challenge: Option<String>,
    }

    warp::get()
        .and(warp::query::query())
        .and(view::with_tera())
        .and(with_hydra_api())
                .map(
                    move |query_params: LoginQueryParams, tera: Tera, hydra_api: AdminApiClient| {

                        // The challenge is used to fetch information about the login request from ORY Hydra.
                        query_params
                            .login_challenge
                            .map(|login_challenge| {
                                let login_request =
                                    hydra_api.get_login_request(&login_challenge).unwrap();

                                // If hydra was already able to authenticate the user, skip will be true and we do not need to re-authenticate
                                if login_request.skip.unwrap_or(false) {
                                    info!("Hydra was already able to authenticate the user, skipping login as we do not need to re-authenticate");
                                    info!("Accepting login request with Hydra");

                                    // You can apply logic here, for example update the number of times the user logged in.
                                    // ...

                                    // Now it's time to grant the login request. You could also deny the request if something went terribly wrong
                                    // (e.g. your arch-enemy logging in...)
                                    let completed_request = hydra_api
                                        .accept_login_request(
                                            &login_challenge,
                                            Some(hydra::models::AcceptLoginRequest::new(
                                                // All we need to do is to confirm that we indeed want to log in the user.
                                                // We are using a hardcoded subject here, the subject should be an immutable id of the user that is loggin in
                                                // to let Hydra know which user to associate with this login.
                                                "hardcoded_subject".to_owned(),
                                            )),
                                        )
                                        .unwrap();

                                    // All we need to do now is to redirect the user back to hydra!
                                    Box::new(warp::redirect(
                                        Uri::from_str(
                                            &completed_request
                                                .redirect_to
                                                .unwrap_or("/".to_owned()),
                                        )
                                        .unwrap(),
                                    )) as Box<dyn warp::Reply>
                                } else {
                                    // If authentication can't be skipped we MUST show the login UI.
                                    info!("Sending user to login");

                                    // The challenge will be a hidden input field
                                    let mut context = Context::new();
                                    context.insert("login_challenge", &login_challenge);

                                    let body = tera.render("login.html", &context).unwrap();
                                    Box::new(warp::reply::html(body)) as Box<dyn warp::Reply>
                                }
                            })
                            .unwrap_or_else(|| {
                                let body = tera.render("login.html", &Context::new()).unwrap();
                                Box::new(warp::reply::html(body)) as Box<dyn warp::Reply>
                            })
                    },
                )
        .boxed()
}

pub fn accept_login() -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct FormBody {
        login_challenge: Option<String>,
        username: String,
        password: String,
    }

    warp::post()
        .and(
            warp::body::content_length_limit(1024 * 32)
                .and(warp::body::form())
                .and(with_hydra_api())
                .map(|form_body: FormBody, hydra_api: AdminApiClient| {
                    // Add logic here to verify the username and password from the submitted login form

                    // Accepting login request, although you could still deny the login request if something else went wrong
                    form_body
                        .login_challenge
                        .map(|login_challenge| {
                            info!("Accepting login request with Hydra");
                            let completed_request = hydra_api
                                .accept_login_request(
                                    &login_challenge,
                                    Some(hydra::models::AcceptLoginRequest::new(
                                        // We are using a hardcoded subject here, the subject should be an immutable id of the user that is loggin in
                                        // to let Hydra know which user to associate with this login
                                        "hardcoded_subject".to_owned(),
                                    )),
                                )
                                .unwrap();

                            // Redirecting to hydra
                            warp::redirect(
                                Uri::from_str(
                                    &completed_request.redirect_to.unwrap_or("/".to_owned()),
                                )
                                .unwrap(),
                            )
                        })
                        .unwrap_or_else(|| warp::redirect(Uri::from_str("/").unwrap()))
                }),
        )
        .boxed()
}

pub fn consent_page() -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct ConsentContext {
        consent_challenge: String,
        client_id: String,
        client_name: Option<String>,
        requested_scopes: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct QueryParams {
        consent_challenge: String,
    }

    warp::get()
        .and(warp::query())
        .and(view::with_tera())
        .and(with_hydra_api())
        .map(
            move |query_params: QueryParams, tera: Tera, hydra_api: AdminApiClient| {
                // The challenge is used to fetch information about the consent request from ORY Hydra.
                let consent_request = hydra_api
                    .get_consent_request(&query_params.consent_challenge)
                    .unwrap();

                // If a user has granted this application the requested scope, hydra will tell us to not show the UI.
                if consent_request.skip.unwrap_or(false) {
                    let completed_request = hydra_api
                        .accept_consent_request(
                            &query_params.consent_challenge,
                            Some(hydra::models::AcceptConsentRequest {
                                // ORY Hydra checks if requested audiences are allowed by the client, so we can simply echo this.
                                grant_access_token_audience: consent_request
                                    .requested_access_token_audience,
                                // We can grant all scopes that have been requested - hydra already checked for us that no additional scopes
                                // are requested accidentally.
                                grant_scope: consent_request.requested_scope,

                                remember: None,
                                remember_for: None,

                                // The session allows us to set session data for id and access tokens
                                // This data will be available when introspecting the token. Try to avoid sensitive information here,
                                // unless you limit who can introspect tokens.
                                // access_token: { foo: 'bar' },

                                // This data will be available in the ID token.
                                // id_token: { baz: 'bar' },
                                session: None,
                            }),
                        )
                        .unwrap();
                    // All we need to do now is to redirect the user back to hydra!
                    Box::new(warp::redirect(
                        Uri::from_str(&completed_request.redirect_to.unwrap_or("/".to_owned()))
                            .unwrap(),
                    )) as Box<dyn warp::Reply>
                } else {
                    // If consent can't be skipped we MUST show the consent UI.

                    let oauth2_client = consent_request
                        .client
                        .expect("no client associated with consent request");
                    let context = ConsentContext {
                        // The challenge will be a hidden input field
                        consent_challenge: query_params.consent_challenge,
                        client_id: oauth2_client.client_id.unwrap(),
                        client_name: oauth2_client.client_name,
                        requested_scopes: consent_request
                            .requested_scope
                            .unwrap_or(vec![])
                            .join(","),
                    };
                    let body = tera
                        .render("consent.html", &Context::from_serialize(&context).unwrap())
                        .unwrap();
                    Box::new(warp::reply::html(body)) as Box<dyn warp::Reply>
                }
            },
        )
        .boxed()
}

pub fn accept_consent() -> warp::filters::BoxedFilter<(impl warp::reply::Reply,)> {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    enum ConsentStatus {
        Authorize,
        Deny,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct FormBody {
        consent_challenge: String,
        submit: ConsentStatus,
    }

    warp::post()
        .and(
            warp::body::content_length_limit(1024 * 32)
                .and(warp::body::form())
                .and(with_hydra_api())
                .map(|form_body: FormBody, hydra_api: AdminApiClient| {
                    let consent_challenge = form_body.consent_challenge;

                    // The challenge is used to fetch information about the consent request from ORY Hydra.
                    let consent_request =
                        hydra_api.get_consent_request(&consent_challenge).unwrap();

                    match form_body.submit {
                        ConsentStatus::Authorize => {
                            let completed_request = hydra_api
                                .accept_consent_request(
                                    &consent_challenge,
                                    Some(hydra::models::AcceptConsentRequest {
                                        // ORY Hydra checks if requested audiences are allowed by the client, so we can simply echo this.
                                        grant_access_token_audience: consent_request
                                            .requested_access_token_audience,
                                        // We can grant all scopes that have been requested - hydra already checked for us that no additional scopes
                                        // are requested accidentally.
                                        grant_scope: consent_request.requested_scope,

                                        remember: None,
                                        remember_for: None,

                                        // The session allows us to set session data for id and access tokens
                                        // This data will be available when introspecting the token. Try to avoid sensitive information here,
                                        // unless you limit who can introspect tokens.
                                        // access_token: { foo: 'bar' },

                                        // This data will be available in the ID token.
                                        // id_token: { baz: 'bar' },
                                        session: None,
                                    }),
                                )
                                .unwrap();
                            // All we need to do now is to redirect the user back to hydra!
                            warp::redirect(
                                Uri::from_str(
                                    &completed_request.redirect_to.unwrap_or("/".to_owned()),
                                )
                                .unwrap(),
                            )
                        }
                        ConsentStatus::Deny => {
                            let completed_request = hydra_api
                                .reject_consent_request(
                                    &consent_challenge,
                                    Some(hydra::models::RejectRequest {
                                        error: Some("access_denied".to_owned()),
                                        error_debug: None,
                                        error_description: Some(
                                            "The resource owner denied the request".to_owned(),
                                        ),
                                        error_hint: None,
                                        status_code: None,
                                    }),
                                )
                                .unwrap();

                            warp::redirect(
                                Uri::from_str(
                                    &completed_request.redirect_to.unwrap_or("/".to_owned()),
                                )
                                .unwrap(),
                            )
                        }
                    }
                }),
        )
        .boxed()
}
