use crate::routes::TheRouter;
use leptos::*;
use leptos_meta::*;
use leptos_router::Router;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Title formatter=|text| format!("Taak | {text}")/>

        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <main>
                <TheRouter/>
            </main>
        </Router>
    }
}
