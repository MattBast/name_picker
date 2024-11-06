use crate::components::NameCard;
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
        });
        names.push(Person {
            id: Uuid::new_v4(),
            name: RwSignal::new(String::from("Hanako")),
        });
        names.push(Person {
            id: Uuid::new_v4(),
            name: RwSignal::new(String::from("Michiko")),
        });
    });

    let emoji_list = get_emojis();

    view! {
        <div class="min-h-screen w-full flex items-center justify-center p-4">
            <CardGrid people emoji_list/>
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
                    on_keyboard_event=move |ev| {
                        if ev.key() == "Enter" || ev.key() == "Tab" {
                            ev.prevent_default();
                            new_card(people)
                        }
                    }

                    on_click_event=move |_| delete_card(people, person.id)
                />
            </For>
            // Button for adding a new card.
            <Button
                appearance=ButtonAppearance::Transparent
                icon=icondata::ChPlus
                on_click=move |_| new_card(people)
            >
                "New"
            </Button>
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

fn new_card(people: RwSignal<Vec<Person>>) {
    people.update(|people| {
        people.push(Person {
            id: Uuid::new_v4(),
            name: RwSignal::new(String::new()),
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
