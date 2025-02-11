use leptos::prelude::*;

#[component]
pub fn Select(value: RwSignal<String>, values: RwSignal<Vec<String>>) -> impl IntoView {
    let select_ref = NodeRef::<leptos::html::Select>::new();

    let on_change = move |_| {
        let Some(el) = select_ref.get() else {
            return;
        };
        value.set(el.value());
    };

    view! {
        <select
            on:change=on_change
            node_ref=select_ref
            class="
                w-full py-3.5 text-center text-base rounded-lg
                border border-transparent border-r-8 shadow-sm
                outline active:outline-slate-900 hover:outline-slate-900
                active:outline-2 hover:outline-2
            "
        >
            <For each=move || values.get() key=|v| v.clone() let:v>
                <option>{v}</option>
            </For>
        </select>
    }
}
