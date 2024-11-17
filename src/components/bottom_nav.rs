use crate::components::FilledButton;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use thaw::Icon;

#[component]
pub fn BottomNav<F, F2, F3>(
    spin_function: F,
    new_function: F2,
    reset_function: F3,
    picked: RwSignal<bool>,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
    F2: Fn(MouseEvent) + 'static,
    F3: Fn(MouseEvent) + 'static,
{
    view! {
        <div class="w-full py-4 bg-yellow-300">
            <div class="flex justify-center items-center gap-2 m-3 w-full">
                <div class: hidden=move || picked.get()>
                    <FilledButton on_click=spin_function>
                        <Icon icon=icondata::FaDiceSolid/>
                        "Spin"
                    </FilledButton>
                </div>
                <div class: hidden=move || picked.get()>
                    <FilledButton on_click=new_function>
                        <Icon icon=icondata::AiPlusOutlined/>
                        "Add a name"
                    </FilledButton>
                </div>
                <div class: hidden=move || !picked.get()>
                    <FilledButton on_click=reset_function>
                        <Icon icon=icondata::BsArrowCounterclockwise/>
                        "Reset"
                    </FilledButton>
                </div>
            </div>
        </div>
    }
}
