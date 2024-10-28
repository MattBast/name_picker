use crate::pages::*;
use leptos::*;
use leptos_router::components::{Route, Routes};
use leptos_router::path;

#[component]
pub fn TheRouter() -> impl IntoView {
    view! {
        <Routes fallback=|| "404">
            <Route path=path!("/") view=Home/>
        </Routes>
    }
}
