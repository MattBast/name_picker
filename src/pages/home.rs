use crate::components::{start_confetti, BottomNav, FilledButton, NameCard};
use crate::data::Person;
use leptos::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use thaw::*;
use uuid::Uuid;

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

#[component]
pub fn CardGrid(people: RwSignal<Vec<Person>>, emoji_list: Vec<String>) -> impl IntoView {
    view! {
        <Flex gap=FlexGap::Small justify=FlexJustify::Center class="flex-wrap">
            // Create one card for every name.
            <For each=move || people.get() key=|person| person.id let:person>
                <NameCard
                    name=person.name
                    emoji_list=emoji_list.clone()
                    picked=person.picked
                    not_picked=person.not_picked
                    on_keyboard_event=move |ev| {
                        if ev.key() == "Tab" && ev.shift_key() {
                            ev.prevent_default();
                            prev_card(people, person.id)
                        }
                        if ev.key() == "Enter" || (ev.key() == "Tab" && !ev.shift_key()) {
                            ev.prevent_default();
                            next_card(people, person)
                        }
                        if ev.key() == "Escape" {
                            ev.prevent_default();
                            if let Some(input) = person.input_ref.get_untracked() {
                                let _ = input.blur();
                            }
                        }
                    }

                    // Delete the card if the user blurs and the input was left empty.
                    on_blur_event=move |_| {
                        if person.name.get_untracked().is_empty() {
                            delete_card(people, person.id)
                        }
                    }

                    on_click_event=move |_| delete_card(people, person.id)
                    node_ref=person.input_ref
                />
            </For>
        </Flex>
    }
}

fn get_emojis() -> Vec<String> {
    emojis::Group::SmileysAndEmotion
        .emojis()
        .filter(|e| e.unicode_version() < emojis::UnicodeVersion::new(13, 0))
        .map(|e| e.as_str().to_owned())
        // .take(30)
        .collect()
}

fn next_card(people: RwSignal<Vec<Person>>, current_person: Person) {
    let current_index = people
        .get_untracked()
        .iter()
        .position(|p| p.id == current_person.id)
        .unwrap();

    // Check if there's a next card and focus on it if there is one.
    if current_index < people.get_untracked().len() - 1 {
        let next_person_ref = people.get_untracked()[current_index + 1].input_ref;
        if let Some(input) = next_person_ref.get_untracked() {
            let _ = input.focus();
        }
    } else {
        // Create new card if we're at the last position so long as the current
        // input isn't empty.
        if !current_person.name.get_untracked().is_empty() {
            new_card(people)
        }
    }
}

fn prev_card(people: RwSignal<Vec<Person>>, current_id: Uuid) {
    let current_index = people
        .get_untracked()
        .iter()
        .position(|p| p.id == current_id)
        .unwrap();

    // Check if there's a previous card and focus on it if there is one.
    // Otherwise do nothing.
    if current_index != 0 {
        let next_person_ref = people.get_untracked()[current_index - 1].input_ref;
        if let Some(input) = next_person_ref.get_untracked() {
            let _ = input.focus();
        }
    }
}

fn new_card(people: RwSignal<Vec<Person>>) {
    people.update(|people| {
        people.push(Person {
            id: Uuid::new_v4(),
            name: RwSignal::new(String::new()),
            input_ref: NodeRef::new(),
            picked: RwSignal::new(false),
            not_picked: RwSignal::new(false),
        })
    })
}

fn delete_card(people_signal: RwSignal<Vec<Person>>, id: Uuid) {
    people_signal.update(|people| {
        if let Some(index) = people.iter().position(|person| person.id == id) {
            // Delete the card.
            let _ = people.remove(index);

            // Check if there's a previous card and focus on it if there is one.
            // Otherwise do nothing.
            if index != 0 {
                let prev_person_ref = people[index - 1].input_ref;
                if let Some(input) = prev_person_ref.get_untracked() {
                    let _ = input.focus();
                }
            }
        };
    })
}

fn random_card(people: RwSignal<Vec<Person>>, picked: RwSignal<bool>) {
    // Randomly pick a person from the list.
    match people.get_untracked().choose(&mut thread_rng()) {
        Some(picked_person) => {
            // Hide the spin and new name buttons.
            picked.set(true);
            // Set the selected person as picked so their style highlights
            // them to the user.
            picked_person.picked.set(true);
            // Scroll to the the picked persons card.
            if let Some(input) = picked_person.input_ref.get_untracked() {
                let _ = input.scroll_into_view();
            }
            // Set all other people to not_picked so their style hides
            // them from the user.
            people.update(|p| {
                p.iter_mut().for_each(|person| {
                    if person.id != picked_person.id {
                        person.not_picked.set(true)
                    }
                })
            })
        }
        // Placeholder log if the list was empty.
        None => log!("Failed to pick someone."),
    }
}

fn reset_cards(people: RwSignal<Vec<Person>>, picked: RwSignal<bool>) {
    people.update(|p| {
        // Hide the reset button.
        picked.set(false);
        // Set all people to neither picked or not picked.
        p.iter_mut().for_each(|person| {
            person.picked.set(false);
            person.not_picked.set(false);
        })
    })
}
