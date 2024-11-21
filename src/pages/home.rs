use crate::components::{start_confetti, BottomNav, CardGrid, FilledButton};
use crate::utils::*;
use leptos::prelude::*;
use thaw::*;

#[component]
pub fn Home() -> impl IntoView {
    let people = RwSignal::new(Vec::new());

    let emoji_list = get_emojis();

    let picked = RwSignal::new(false);

    let confetti_container = NodeRef::<leptos::html::Div>::new();

    view! {
        <div class="h-screen w-full flex flex-col border-t-2 border-yellow-400">

            // Generate confetti when the `start_confetti` function is called
            <div
                node_ref=confetti_container
                style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 1000;"
            />

            // A container for the list of names. This container is hidden
            // if the list contains no names.
            <div
                class="flex-1 w-full flex justify-center items-start p-10 overflow-y-auto"
                class: hidden=move || people.get().is_empty()
            >
                <CardGrid people emoji_list/>
            </div>

            // Contains the buttons the user can use to add names, select random
            // ones and reset the random selection.
            <BottomNav people>
                // Button to randomly select a name from the list. Is hidden if
                // a random name has already been selected i.e. the button was
                // clicked, or there are no names to pick from.
                <div class: hidden=move || picked.get() || (people.get().len() < 2)>
                    <FilledButton on_click=move |_| {
                        random_card(people, picked);
                        start_confetti(confetti_container.clone());
                    }>
                        <Icon icon=icondata::FaDiceSolid class="w-5 h-5 mr-1.5"/>
                        "Spin"
                    </FilledButton>
                </div>
                // Button to add a new name to the list. Clicking the button adds
                // a new card to the people vector which should focus the user on
                // its input field.
                <div class: hidden=move || picked.get()>
                    <FilledButton on_click=move |_| new_card(people)>
                        <Icon icon=icondata::AiPlusOutlined class="w-4 h-4 mr-1.5"/>
                        "Add a name"
                    </FilledButton>
                </div>
                // This button is only shown if a random name has been picked. This
                // button unpicks the name so the user can add more names or randomly
                // pick another.
                <div class: hidden=move || !picked.get()>
                    <FilledButton on_click=move |_| reset_cards(people, picked)>
                        <Icon icon=icondata::BsArrowCounterclockwise class="w-4 h-4 mr-1.5"/>
                        "Reset"
                    </FilledButton>
                </div>
            </BottomNav>
        </div>
    }
}
