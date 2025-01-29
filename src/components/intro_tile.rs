use crate::data::Person;
use leptos::prelude::*;

#[component]
pub fn IntroTile(people: RwSignal<Vec<Person>>) -> impl IntoView {
    view! {
        <div
            class="w-auto h-full py-4 bg-gradient-to-br from-amber-200 from-20% via-pink-200 to-fuchsia-200 flex flex-col justify-center rounded-md m-2"
            class: hidden=move || !people.get().is_empty()
        >
            // This contains text to explain what the app does to the user.
            <div class="w-full p-4 text-center">
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
        </div>
    }
}
