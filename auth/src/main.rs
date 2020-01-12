use auth;
use warp;

#[tokio::main]
async fn main() {
    if ::std::env::var_os("RUST_LOG").is_none() {
        ::std::env::set_var("RUST_LOG", "warp=info,auth_svc=trace,api_access=trace");
    }
    env_logger::init();

    warp::serve(auth::routes())
        .run(([127, 0, 0, 1], 3000))
        .await;
}
