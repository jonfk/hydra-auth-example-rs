use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use warp::{self, Filter};

pub mod view;

#[tokio::main]
async fn main() {
    let routes = auth_routes().or(homepage());
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
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
        .map(move |query_params: LoginQueryParams, tera: Tera| {
            let mut context = Context::new();
            context.insert("login_challenge", &query_params.login_challenge);

            let body = tera.render("login.html", &context).unwrap();
            warp::reply::html(body)
        })
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
                .map(|form_body: FormBody| warp::reply::json(&form_body)),
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
        .map(move |query_params: QueryParams, tera: Tera| {
            let context = ConsentContext {
                consent_challenge: query_params.consent_challenge,
                client_id: "client_id-1234".to_owned(),
                client_name: Some("client Name".to_owned()),
                requested_scopes: "".to_owned(),
            };

            let body = tera
                .render("consent.html", &Context::from_serialize(context).unwrap())
                .unwrap();

            warp::reply::html(body)
        })
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
                .map(|form_body: FormBody| warp::reply::json(&form_body)),
        )
        .boxed()
}
