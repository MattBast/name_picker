use crate::components::{start_confetti, BottomNav, CardGrid, FilledButton, IntroTile};
use crate::utils::*;
use leptos::prelude::*;
use thaw::{Button, ButtonShape, Icon, Menu, MenuItem, MenuTrigger, MenuTriggerType};

#[component]
pub fn Home() -> impl IntoView {
    let people = RwSignal::new(Vec::new());

    let emoji_list = RwSignal::new(get_emojis(emojis::Group::SmileysAndEmotion));

    let picked = RwSignal::new(false);

    let confetti_container = NodeRef::<leptos::html::Div>::new();

    let selected_emoji_group = RwSignal::new(String::from("😀 Smileys & Emotion"));
    let emoji_groups = RwSignal::new(get_emoji_groups());

    // Declare what happens when the "more" menu items are selected
    let on_emoji_select = move |emoji: String| {
        selected_emoji_group.set(emoji.clone());

        let new_group = emoji_to_group(emoji);
        emoji_list.set(get_emojis(new_group));
    };

    view! {
        <div class="h-screen w-full flex flex-col md:flex-row bg-gray-100">

            // Generate confetti when the `start_confetti` function is called
            <div
                node_ref=confetti_container
                style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 1000;"
            />

            // A tile that contains the website intro or cards depending on
            // whether any card exist or not.
            <div
                class="flex-1 overflow-y-auto p-10 rounded-md m-2 bg-gradient-to-br from-amber-100 from-20% via-pink-100 to-fuchsia-100"
            >
                <div class="w-full flex items-center gap-2 justify-end">
                    // A select component that allows the user to pick the group of emojis
                    // they'd like to see on the cards.
                    <div class="w-fit">
                        <Menu on_select=on_emoji_select trigger_type=MenuTriggerType::Hover>
                            // The element that opens the menu when clicked or hovered.
                            <MenuTrigger slot>
                                <Button
                                    shape=ButtonShape::Circular
                                >
                                    {selected_emoji_group}
                                </Button>
                            </MenuTrigger>
                            <For each=move || emoji_groups.get() key=|v| v.clone() let:v>
                                <MenuItem value={v.clone()}>
                                    {v}
                                </MenuItem>
                            </For>
                        </Menu>
                    </div>

                    // Button to delete all cards on the page.
                    <div class="w-fit">
                        <Button
                            icon=icondata::BiTrashAltSolid
                            shape=ButtonShape::Circular
                            on:click=move |_| clear_cards(people)
                            class: hidden=move || (people.get().len() < 2)
                        />
                    </div>
                </div>

                // An intro page that shows when the user has not yet
                // added any items to the list.
                <IntroTile people/>

                // A container for the list of names. This container is hidden
                // if the list contains no names.
                <CardGrid people emoji_list/>
            </div>

            // Contains the buttons the user can use to add names, select random
            // ones and reset the random selection.
            <BottomNav>
                // Button to randomly select a name from the list. Is hidden if
                // a random name has already been selected i.e. the button was
                // clicked, or there are no names to pick from.
                <div
                    class: hidden=move || picked.get() || (people.get().len() < 2)
                    class="w-40"
                >
                    <FilledButton on_click=move |_| {
                        random_card(people, picked);
                        start_confetti(confetti_container);
                    }>
                        <Icon icon=icondata::FaDiceSolid class="w-5 h-5 mr-1.5"/>
                        "Spin"
                    </FilledButton>
                </div>
                // Button to add a new name to the list. Clicking the button adds
                // a new card to the people vector which should focus the user on
                // its input field.
                <div
                    class: hidden=move || picked.get()
                    class="w-40"
                >
                    <FilledButton on_click=move |_| new_card(people)>
                        <Icon icon=icondata::AiPlusOutlined class="w-4 h-4 mr-1.5"/>
                        "Add a name"
                    </FilledButton>
                </div>
                // This button is only shown if a random name has been picked. This
                // button unpicks the name so the user can add more names or randomly
                // pick another.
                <div
                    class: hidden=move || !picked.get()
                    class="w-40"
                >
                    <FilledButton on_click=move |_| reset_cards(people, picked)>
                        <Icon icon=icondata::BsArrowCounterclockwise class="w-4 h-4 mr-1.5"/>
                        "Reset"
                    </FilledButton>
                </div>
            </BottomNav>
        </div>
    }
}
