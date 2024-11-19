use leptos::prelude::*;

#[component]
pub fn BottomNav(children: Children) -> impl IntoView {
    view! {
        <div class="w-full py-4 bg-yellow-300">
            <div class="flex justify-center items-center gap-2 m-3 w-full">
                {children()}
            </div>
        </div>
    }
}
