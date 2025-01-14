#[macro_use]
extern crate log;
#[macro_use]
extern crate rust_i18n;

mod nats;
mod app;
mod bot;
mod db;

i18n!("locales", fallback = "ru");

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init_timed();

    let _guard = sentry::init(sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    });
}
