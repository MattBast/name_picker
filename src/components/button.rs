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
    let button_text = match text {
        Some(t) => t,
        None => String::new(),
    };

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
            class="rounded-md bg-blue-600 py-2 px-4 border border-transparent text-center text-sm text-white transition-all shadow-md hover:shadow-lg focus:bg-blue-500 focus:shadow-none active:bg-blue-500 hover:bg-blue-500 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none"
            type="button"
            on:click=on_click
        >
            {children()}
        </button>
    }
}
