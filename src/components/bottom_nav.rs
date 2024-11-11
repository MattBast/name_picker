use crate::components::FilledButton;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

#[component]
pub fn BottomNav<F, F2>(spin_function: F, new_name: F2) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
    F2: Fn(MouseEvent) + 'static,
{
    view! {
        <nav class="fixed bottom-0 left-0 right-0 w-full py-4 border-t-2 border-yellow-300 bg-white">
              <div class="flex justify-center items-center gap-2 mt-2 mb-4 w-full">
                    <FilledButton
                        text=Some("Spin".to_string())
                        icon=icondata::FaDiceSolid
                        on_click=spin_function
                    />
                    <FilledButton
                        text=Some("Add a name".to_string())
                        icon=icondata::AiPlusOutlined
                        on_click=new_name
                    />
              </div>
        </nav>
    }
}
