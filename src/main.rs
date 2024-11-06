#![allow(non_snake_case)]

mod app;
mod components;
mod data;
mod pages;
mod routes;

use app::App;
use leptos::mount::mount_to_body;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App)
}
