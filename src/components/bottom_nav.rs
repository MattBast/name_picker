use crate::data::Person;
use leptos::prelude::*;

#[component]
pub fn BottomNav(people: RwSignal<Vec<Person>>, children: Children) -> impl IntoView {
    view! {
        <div
            class="w-full py-4 bg-yellow-300 flex flex-col justify-center"
            class: h-full=move || people.get().is_empty()
        >
            // This contains text to explain what the app does to the user.
            <div
                class="w-full p-4 text-center"
                class: hidden=move || !people.get().is_empty()
            >
                <div
                class="flex justify-center items-center text-4xl gap-4"
                >
                    <div>
                        {emojis::get("😀").unwrap().as_str()}
                    </div>
                    <div class="rotate-45 -translate-y-1 text-6xl">
                        {emojis::get("🥳").unwrap().as_str()}
                    </div>
                    <div>
                        {emojis::get("😎").unwrap().as_str()}
                    </div>
                    <div>
                        {emojis::get("😁").unwrap().as_str()}
                    </div>
                </div>
                <h1
                    class="font-normal text-6xl tracking-wide font-sans subpixel-antialiased text-slate-800"
                >
                    "Name Picker"
                </h1>
                <h2
                    class="font-normal text-3xl tracking-normal leading-normal font-sans antialiased text-slate-600"
                >
                    "Write a list of names and randomly select one."
                </h2>
            </div>

            // This contains the control panel buttons that the user will use to
            // interact with the name picker app.
            <div class="flex justify-center items-center gap-2 w-full">
                {children()}
            </div>
        </div>
    }
}
