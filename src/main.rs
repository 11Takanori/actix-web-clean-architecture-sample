mod domain;
mod driver;
mod error;
mod gateway;
mod port;
mod rest;
mod usecase;

#[macro_use]
extern crate log;

use std::env;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let sys = actix::System::new("actix-web-sample");
    if let Err(e) = rest::build() {
        error!("ERROR: {:?}!", e);
    }

    let _ = sys.run();
}
