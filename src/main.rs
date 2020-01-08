use tera::{Context, Tera};
use warp::{self, path, Filter};

#[tokio::main]
async fn main() {
    let mut tera = Tera::default();
    tera.add_raw_template(
        "hello.html",
        r#"
<!DOCTYPE html>
<head>
    <meta charset="UTF-8">
    <title>Hello</title>
</head>
<body>
<p>Hello, {{ name }}!</p>
</body>
</html>
"#,
    )
    .unwrap();

    let with_tera = warp::any().map(move || tera.clone());

    let hello = path!("hello" / String)
        .and(with_tera)
        .map(|name: String, tera: Tera| {
            let mut context = Context::new();
            context.insert("name", &name);

            let body = tera.render("hello.html", &context).unwrap();
            warp::reply::html(body)
        });

    warp::serve(hello).run(([127, 0, 0, 1], 3000)).await;
}
