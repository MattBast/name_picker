use leptos::prelude::*;

#[component]
pub fn BottomNav(children: Children) -> impl IntoView {
    view! {
        // This contains the control panel buttons that the user will use to
        // interact with the name picker app.
        <div class="w-auto px-4 py-2 m-2 bg-white shadow-md rounded-md">
            <div class="container flex flex-wrap items-center justify-center w-full mx-auto gap-2">
                {children()}
            </div>
        </div>
    }
}
