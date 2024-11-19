use crate::data::Person;
use leptos::prelude::*;

#[component]
pub fn BottomNav(people: RwSignal<Vec<Person>>, children: Children) -> impl IntoView {
    view! {
        <div
            class="w-full py-4 bg-yellow-300"
            class: h-full=move || people.get().is_empty()
        >
            <div class="flex justify-center items-center gap-2 w-full h-full">
                {children()}
            </div>
        </div>
    }
}
