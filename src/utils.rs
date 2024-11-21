use crate::data::Person;
use leptos::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use uuid::Uuid;

pub fn get_emojis() -> Vec<String> {
    emojis::Group::SmileysAndEmotion
        .emojis()
        .filter(|e| e.unicode_version() < emojis::UnicodeVersion::new(13, 0))
        .map(|e| e.as_str().to_owned())
        // .take(30)
        .collect()
}

pub fn new_card(people: RwSignal<Vec<Person>>) {
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

pub fn random_card(people: RwSignal<Vec<Person>>, picked: RwSignal<bool>) {
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

pub fn reset_cards(people: RwSignal<Vec<Person>>, picked: RwSignal<bool>) {
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

pub fn next_card(people: RwSignal<Vec<Person>>, current_person: Person) {
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

pub fn prev_card(people: RwSignal<Vec<Person>>, current_id: Uuid) {
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

pub fn delete_card(people_signal: RwSignal<Vec<Person>>, id: Uuid) {
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
