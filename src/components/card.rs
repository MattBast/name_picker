use crate::components::NameInput;
use leptos::ev::KeyboardEvent;
use leptos::prelude::*;
use rand::seq::SliceRandom;
use thaw::*;

#[component]
pub fn NameCard<F>(
    name: RwSignal<String>,
    emoji_list: Vec<String>,
    on_keyboard_event: F,
) -> impl IntoView
where
    F: Fn(KeyboardEvent) + Send + 'static,
{
    view! {
        <div class="grow-0 shrink-0 basis-20 sm:basis-28 md:basis-40">
            <Card class="w-full h-full p-0 border-slate-200 border-2 rounded">
                // A picture to help identify the name.
                <CardPreview class="bg-blue-200 p-2.5">
                    <div class="flex items-center justify-center text-5xl hover:animate-bounce transition-all duration-75">
                        {random_emoji(emoji_list)}
                    </div>
                </CardPreview>
                // The editable name.
                <div class="p-2.5">
                    <NameInput name on_keyboard_event=on_keyboard_event/>
                </div>
            </Card>
        </div>
    }
}

fn random_emoji(emoji_list: Vec<String>) -> String {
    emoji_list.choose(&mut rand::thread_rng()).unwrap().clone()
}
