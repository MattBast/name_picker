use leptos::ev::MouseEvent;
use leptos::prelude::*;
use thaw::Icon;

#[component]
pub fn TransparentButton<F>(
    text: Option<String>,
    icon: icondata::Icon,
    on_click: F,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let button_text = text.unwrap_or_default();

    view! {
        <button
            class="rounded-md border border-transparent py-2 px-4 text-center text-sm transition-all text-slate-600 hover:bg-slate-100 focus:bg-slate-100 active:bg-slate-100 disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none"
            type="button"
            on:click=on_click
        >
            <Icon icon=icon/>
            {button_text}
        </button>
    }
}

#[component]
pub fn FilledButton<F>(on_click: F, children: Children) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <button
            class="w-full rounded-md bg-slate-900 py-3.5 px-6 border border-transparent text-center text-base text-white transition-all shadow-md hover:shadow-lg focus:bg-slate-800 focus:shadow-none active:bg-slate-800 hover:bg-slate-800 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none"
            type="button"
            on:click=on_click
        >
            {children()}
        </button>
    }
}
