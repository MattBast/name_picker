use crate::components::FilledButton;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use thaw::Icon;

#[component]
pub fn BottomNav<F, F2>(spin_function: F, new_name: F2) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
    F2: Fn(MouseEvent) + 'static,
{
    view! {
        <nav class="fixed bottom-0 left-0 right-0 w-full py-4 border-t-2 border-yellow-300 bg-white">
              <div class="flex justify-center items-center gap-2 mt-2 mb-4 w-full">
                    <FilledButton on_click=spin_function>
                        <Icon icon=icondata::FaDiceSolid/>
                        "Spin"
                    </FilledButton>
                    <FilledButton on_click=new_name>
                        <Icon icon=icondata::AiPlusOutlined/>
                        "Add a name"
                    </FilledButton>
              </div>
        </nav>
    }
}
