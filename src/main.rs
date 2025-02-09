#![allow(non_snake_case)]

mod components;
mod data;
mod pages;
mod utils;
use crate::pages::*;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Title text="Random.Pick"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Router>
            <main>
                <TheRouter/>
            </main>
        </Router>
    }
}
#[component]
pub fn TheRouter() -> impl IntoView {
    view! {
        <Routes fallback=|| "404">
            <Route path=path!("/") view=Home/>
        </Routes>
    }
}
