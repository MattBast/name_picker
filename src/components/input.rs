use leptos::prelude::*;

#[component]
pub fn NameInput(name: RwSignal<String>) -> impl IntoView {
    let input_element: NodeRef<leptos::html::Input> = NodeRef::new();

    // Focus on the input element after the card is created.
    Effect::new(move |_| {
        if let Some(input) = input_element.get_untracked() {
            let _ = input.focus();
        }
    });

    view! {
        <div class="relative group">
            // The input where the user types a name.
            <input
                type="text"
                class="
                    w-full px-4 py-2 text-gray-700 bg-white outline-none text-center
                "
                placeholder="Your name..."
                value=name
                node_ref=input_element
                // it's a strange hack but the underline animation that triggers
                // on a focus event doesn't trigger unless this focus listener
                // is added (even though it's returning none).
                on:focus=move |_| {
                    ()
                }
            />
            // The underline animation that appears when the user focusses on the
            // input.
            <div
                class="
                    absolute bottom-0 left-0 h-0.5 bg-blue-500
                    w-0 transition-[width] ease-in-out duration-300
                    group-focus-within:w-full group-focus-visible:w-full
                "
            />
        </div>
    }
}