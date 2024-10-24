use crate::pages::*;
use leptos::*;
use leptos_router::{Route, Routes};

#[component]
pub fn TheRouter() -> impl IntoView {
    view! {
        <Routes>
            <Route path="/" view=Home/>
        </Routes>
    }
}
