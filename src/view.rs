use tera::Tera;
use warp::{self, Filter};

pub fn with_tera() -> warp::filters::BoxedFilter<(Tera,)> {
    warp::any().map(move || tera_templates()).boxed()
}

pub fn tera_templates() -> Tera {
    let login_tpl = r#"
<!DOCTYPE html>
<head>
    <meta charset="UTF-8">
    <title>Login</title>
</head>
<body>
<h1>Login page</h1>
<form action="/login" method="post">
{% if login_challenge %}
    <input type="hidden" name="login_challenge" value="{{login_challenge}}"/>
{% else %}
{% endif %}

    <label for="username">Username</label>:
    <input type="text" id="username" name="username" autofocus="autofocus"/> <br/>

    <label for="password">Password</label>:
    <input type="password" id="password" name="password"/> <br/>

    <input type="submit" value="Log in"/>
</form>
</body>
</html>
"#;

    let consent_tpl = r#"
<!DOCTYPE html>
<head>
    <meta charset="UTF-8">
    <title>Consent</title>
</head>
<body>
<p th:if="${error}" class="error">Invalid Consent</p>
<div>
    <p>Client ID: <span>{{ client_id }}</span></p>
    <p>Client Name: <span>{{ client_name | default(value="Unknown") }}</span></p>
    <p>Requested Scopes: <span>{{ requested_scopes }}</span></p>
</div>
<form action="/consent" method="post">
    <input type="hidden" name="consent_challenge" value="{{ consent_challenge }}"/>
    <input type="submit" name="submit" value="Authorize"/>
    <input type="submit" name="submit" value="Deny"/>
</form>
</body>
</html>
"#;

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("login.html", login_tpl),
        ("consent.html", consent_tpl),
    ])
    .unwrap();
    tera
}
