use crate::components::{BottomNav, NameCard};
use crate::data::Person;
use leptos::prelude::*;
use thaw::*;
use uuid::Uuid;

#[component]
pub fn Home() -> impl IntoView {
    let people = RwSignal::new(Vec::new());
    people.update(|names| {
        names.push(Person {
            id: Uuid::new_v4(),
            name: RwSignal::new(String::from("Saburo")),
            input_ref: NodeRef::new(),
        });
        names.push(Person {
            id: Uuid::new_v4(),
            name: RwSignal::new(String::from("Hanako")),
            input_ref: NodeRef::new(),
        });
        names.push(Person {
            id: Uuid::new_v4(),
            name: RwSignal::new(String::from("Michiko")),
            input_ref: NodeRef::new(),
        });
    });

    let emoji_list = get_emojis();

    view! {
        <div class="min-h-screen w-full flex items-center justify-center p-4">
            <CardGrid people emoji_list/>
        </div>

        <BottomNav
            spin_function=move |_| {log!("Clicked spin")}
            new_name=move |_| new_card(people)
        />
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
                    on_keyboard_event=move |ev| {
                        if ev.key() == "Tab" && ev.shift_key() {
                            ev.prevent_default();
                            prev_card(people, person.id)
                        }
                        if ev.key() == "Enter" || (ev.key() == "Tab" && !ev.shift_key()) {
                            ev.prevent_default();
                            next_card(people, person.id)
                        }
                    }

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

fn next_card(people: RwSignal<Vec<Person>>, current_id: Uuid) {
    let current_index = people
        .get_untracked()
        .iter()
        .position(|p| p.id == current_id)
        .unwrap();

    // Check if there's a next card and focus on it if there is one.
    if current_index < people.get().len() - 1 {
        let next_person_ref = people.get_untracked()[current_index + 1].input_ref;
        if let Some(input) = next_person_ref.get_untracked() {
            let _ = input.focus();
        }
    } else {
        // Create new card if we're at the last position
        new_card(people)
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
        })
    })
}

fn delete_card(people: RwSignal<Vec<Person>>, id: Uuid) {
    people.update(|people| {
        if let Some(index) = people.iter().position(|person| person.id == id) {
            let _ = people.remove(index);
        };
    })
}
