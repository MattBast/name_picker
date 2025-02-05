use leptos::prelude::*;

#[component]
pub fn BottomNav(children: Children) -> impl IntoView {
    view! {
        // This contains the control panel buttons that the user will use to
        // interact with the name picker app.
        <div class="w-auto px-4 py-2 m-2 bg-white rounded-md">
            <div class="container flex flex-wrap flex-row md:flex-col items-center justify-center w-full h-full mx-auto gap-2">
                {children()}
            </div>
        </div>
    }
}
