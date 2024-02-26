use axum::{
    routing::{get, post},
    Router,
};
use once_cell::sync::Lazy;
use rand::distributions::{Alphanumeric, DistString};
use routes::{authorize, protected};
use token::Keys;

mod claims;
mod error;
mod routes;
mod token;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = Alphanumeric.sample_string(&mut rand::thread_rng(), 60);
    Keys::new(secret.as_bytes())
});

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/protected", get(protected))
        .route("/login", post(authorize));

    Ok(router.into())
}
