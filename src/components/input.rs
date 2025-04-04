use leptos::ev::{FocusEvent, KeyboardEvent};
use leptos::prelude::*;

#[component]
pub fn NameInput<F, B>(
    name: RwSignal<String>,
    on_keyboard_event: F,
    on_blur_event: B,
    node_ref: NodeRef<leptos::html::Input>,
) -> impl IntoView
where
    F: Fn(KeyboardEvent) + 'static,
    B: Fn(FocusEvent) + 'static,
{
    // Focus on the input element after the card is created.
    Effect::new(move |_| {
        if let Some(input) = node_ref.get_untracked() {
            let _ = input.focus();
        }
    });

    view! {
        <div class="relative group">
            // The input where the user types a name.
            <input
                type="text"
                class="
                w-full px-4 pb-2 pt-4 text-gray-700 bg-white outline-none
                "
                placeholder="Your name..."
                // Change the value of the person name to match the value in the input
                on:input=move |ev| {
                    name.set(event_target_value(&ev));
                }

                prop:value=name
                node_ref=node_ref
                // it's a strange hack but the underline animation that triggers
                // on a focus event doesn't trigger unless this focus listener
                // is added (even though it's returning none).
                on:focus=move |_| { }
                // Bubble up a blue event
                on:blur=on_blur_event
                // Bubble up a keydown event
                on:keydown=on_keyboard_event
            />
            // The underline animation that appears when the user focusses on the
            // input.
            <div class="
            absolute bottom-0 left-0 h-0.5 bg-blue-400
            w-0 transition-[width] ease-in-out duration-300
            group-focus-within:w-full group-focus-visible:w-full
            "></div>
        </div>
    }
}
