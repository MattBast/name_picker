use crate::components::NameCard;
use crate::data::Person;
use crate::utils::*;
use leptos::prelude::*;

#[component]
pub fn CardGrid(people: RwSignal<Vec<Person>>, emoji_list: RwSignal<Vec<String>>) -> impl IntoView {
    view! {
        <div
            class="flex flex-wrap gap-1 justify-center w-full"
            class: hidden=move || people.get().is_empty()
        >
            // Create one card for every name.
            <For each=move || people.get() key=|person| person.id let:person>
                <NameCard
                    name=person.name
                    emoji_list=emoji_list
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
        </div>
    }
}
