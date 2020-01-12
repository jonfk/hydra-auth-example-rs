use auth::{ConsentFormBody, ConsentStatus, LoginFormBody};

use hydra::apis::{configuration::Configuration, AdminApi, AdminApiClient};
use oauth2::{
    basic::BasicClient as OAuth2BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode,
    ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse,
    TokenUrl,
};
use reqwest::{self, Url};
use std::sync::Arc;
use uuid::Uuid;

const AUTHN_BASE_URL: &'static str = "http://localhost:3000";
const HYDRA_ADMIN_BASE_URL: &'static str = "http://localhost:4445";

#[test]
fn login_and_consent_flow() {
    check_auth_svc(AUTHN_BASE_URL);
    check_hydra(HYDRA_ADMIN_BASE_URL);

    let mut config = Configuration::new();
    config.base_path = HYDRA_ADMIN_BASE_URL.to_owned();
    let hydra_admin_client = AdminApiClient::new(Arc::new(config));

    let oauth2_client = create_oauth2_client(&hydra_admin_client);

    let access_token = initiate_oauth2_code_flow(
        &oauth2_client.client_id.unwrap(),
        &oauth2_client.client_secret.unwrap(),
    );

    introspect_access_token(&hydra_admin_client, &access_token);
}

fn check_auth_svc(authn_url: &str) {
    let res = reqwest::blocking::get(authn_url).unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
}

fn check_hydra(hydra_url: &str) {
    let hydra_url = Url::parse(hydra_url).unwrap();
    let res = reqwest::blocking::get(hydra_url.join("/health/ready").unwrap()).unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
}

fn introspect_access_token(hydra_admin_client: &AdminApiClient, access_token: &str) {
    let introspection_res = hydra_admin_client
        .introspect_o_auth2_token(access_token, None)
        .unwrap();
    dbg!(&introspection_res);
    assert!(introspection_res.active);
}

fn create_oauth2_client(hydra_admin_client: &AdminApiClient) -> hydra::models::OAuth2Client {
    let mut new_oauth2_client = hydra::models::OAuth2Client::new();
    new_oauth2_client.client_id = Some(format!(
        "{}-{}",
        Uuid::new_v4().to_string(),
        "my-test-client"
    ));
    new_oauth2_client.client_name = Some("login-flow-test-client".to_owned());
    new_oauth2_client.client_secret = Some("client-secret".to_owned());
    new_oauth2_client.grant_types = Some(vec!["authorization_code".to_owned()]);
    new_oauth2_client.redirect_uris = Some(vec![AUTHN_BASE_URL.to_owned()]);
    new_oauth2_client.token_endpoint_auth_method = Some("client_secret_basic".to_owned());
    new_oauth2_client.scope = Some("openid".to_owned());

    hydra_admin_client
        .create_o_auth2_client(new_oauth2_client)
        .unwrap()
}

fn initiate_oauth2_code_flow(client_id: &str, client_secret: &str) -> String {
    // Create an HTTP client to act as the browser
    let reqwest_client = reqwest::blocking::Client::builder()
        // Enable the cookie store for hydra csrf cookies
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = OAuth2BasicClient::new(
        ClientId::new(client_id.to_owned()),
        Some(ClientSecret::new(client_secret.to_owned())),
        AuthUrl::new("http://localhost:4444/oauth2/auth".to_owned()).unwrap(),
        Some(TokenUrl::new("http://localhost:4444/oauth2/token".to_owned()).unwrap()),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_url(RedirectUrl::new("http://localhost:3000/".to_string()).unwrap());

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("openid".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should usually redirect the user to,
    // in order to trigger the authorization process.
    println!("URL to trigger authorization process: \n{}", auth_url);

    let hydra_login_completed_url = perform_login_flow(&reqwest_client, &auth_url);

    let redirected_url = perform_consent_flow(&reqwest_client, &hydra_login_completed_url);

    let auth_code = Url::parse(&redirected_url)
        .unwrap()
        .query_pairs()
        .find(|c| c.0.as_ref().eq_ignore_ascii_case("code"))
        .unwrap()
        .1
        .to_owned()
        .to_string();

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_state`.
    // Here we are skipping this check for the purposes of brevity of this test.

    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(auth_code.to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request(http_client)
        .unwrap();
    println!("\nOnce the authorization code is exchanged, we finally received an access token");
    dbg!(&token_result);
    println!("{:?}", token_result.access_token().secret());
    token_result.access_token().secret().to_owned()
}

fn perform_login_flow(reqwest_client: &reqwest::blocking::Client, auth_url: &Url) -> String {
    let hydra_authz_res = reqwest_client.get(auth_url.as_ref()).send().unwrap();
    println!("\nThis is the response from Hydra when visiting the url to trigger the authorization process");
    dbg!(&hydra_authz_res);

    assert!(hydra_authz_res.status().is_redirection());

    let login_url = hydra_authz_res
        .headers()
        .get(reqwest::header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();
    let login_challenge = Url::parse(login_url)
        .unwrap()
        .query_pairs()
        .find(|c| c.0.as_ref().eq_ignore_ascii_case("login_challenge"))
        .unwrap()
        .1
        .to_owned()
        .to_string();

    let login_res = reqwest_client
        .post("http://localhost:3000/login")
        .form(&LoginFormBody {
            login_challenge: Some(login_challenge),
            username: "username".to_owned(),
            password: "pass".to_owned(),
        })
        .send()
        .unwrap();

    println!("\nThis is the response from our auth service when visiting the login page");
    dbg!(&login_res);

    assert!(login_res.status().is_redirection());

    let hydra_redirected_url = login_res
        .headers()
        .get(reqwest::header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();

    hydra_redirected_url.to_owned()
}

fn perform_consent_flow(
    reqwest_client: &reqwest::blocking::Client,
    hydra_login_completed_url: &str,
) -> String {
    let hydra_consent_res = reqwest_client
        .get(hydra_login_completed_url)
        .send()
        .unwrap();

    println!("\nThis is the response from hydra when visiting the url received at the end of the login process");
    dbg!(&hydra_consent_res);
    println!("\nAs we can see, hydra will now redirect us to our consent page since the login request was accepted");

    assert!(hydra_consent_res.status().is_redirection());

    let consent_url = hydra_consent_res
        .headers()
        .get(reqwest::header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();
    let consent_challenge = Url::parse(consent_url)
        .unwrap()
        .query_pairs()
        .find(|c| c.0.as_ref().eq_ignore_ascii_case("consent_challenge"))
        .unwrap()
        .1
        .to_owned()
        .to_string();

    let consent_res = reqwest_client
        .post("http://localhost:3000/consent")
        .form(&ConsentFormBody {
            consent_challenge: consent_challenge,
            submit: ConsentStatus::Authorize,
        })
        .send()
        .unwrap();

    println!("\nThis is the response from our auth service when visiting the consent page");
    dbg!(&consent_res);

    assert!(consent_res.status().is_redirection());

    let hydra_consent_approved_url = consent_res
        .headers()
        .get(reqwest::header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();

    let hydra_consent_approved_res = reqwest_client
        .get(hydra_consent_approved_url)
        .send()
        .unwrap();
    println!("\nThis is the response from hydra once the consent request was accepted. You can note that it contains the authorization code appended to the redirect url configured at the start of the OAuth2 process");
    dbg!(&hydra_consent_approved_res);

    let redirected_url = hydra_consent_approved_res
        .headers()
        .get(reqwest::header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();

    redirected_url.to_owned()
}
