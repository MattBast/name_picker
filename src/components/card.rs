use crate::components::NameInput;
use leptos::ev::{FocusEvent, KeyboardEvent, MouseEvent};
use leptos::prelude::*;
use rand::seq::SliceRandom;
use thaw::*;

#[component]
pub fn NameCard<F, M, B>(
    name: RwSignal<String>,
    emoji_list: Vec<String>,
    on_keyboard_event: F,
    on_click_event: M,
    on_blur_event: B,
    node_ref: NodeRef<leptos::html::Input>,
) -> impl IntoView
where
    F: Fn(KeyboardEvent) + Send + 'static,
    M: Fn(MouseEvent) + Send + 'static,
    B: Fn(FocusEvent) + 'static,
{
    view! {
        <div class="relative flex flex-col md:flex-row w-full my-2 bg-white shadow-sm border border-slate-200 rounded-lg w-96">
            // The random emoji that provides a visual identity for the person.
            <div class="relative px-4 py-2.5 text-5xl flex items-center justify-center">
                {random_emoji(emoji_list)}
            </div>
            <div class="flex items-center w-full justify-between">
                // The editable persons name.
                <h4 class="mb-2 text-slate-800 text-xl font-semibold w-full">
                    <NameInput name on_keyboard_event on_blur_event node_ref/>
                </h4>
                // The delete button.
                <div class="pr-2">
                    <Button
                        size=ButtonSize::Large
                        icon=icondata::AiCloseOutlined
                        on:click=on_click_event
                    />
                </div>
            </div>
        </div>
    }
}

fn random_emoji(emoji_list: Vec<String>) -> String {
    emoji_list.choose(&mut rand::thread_rng()).unwrap().clone()
}
