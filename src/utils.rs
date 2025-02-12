use crate::data::Person;
use leptos::logging::log;
use leptos::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use uuid::Uuid;

pub fn get_emojis(group: emojis::Group) -> Vec<String> {
    group
        .emojis()
        .filter(|e| e.unicode_version() < emojis::UnicodeVersion::new(13, 0))
        .map(|e| e.as_str().to_owned())
        .collect()
}

pub fn get_emoji_groups() -> Vec<String> {
    vec![
        String::from("😀 Smileys & Emotion"),
        String::from("🤦 People & Body"),
        String::from("🐶 Animals & Nature"),
        String::from("🍇 Food & Drink"),
        String::from("✈️ Travel & Places"),
        String::from("🎈 Activities"),
        String::from("👘 Objects"),
        String::from("✅ Symbols"),
        String::from("🏁 Flags"),
    ]
}

pub fn emoji_to_group(emoji_str: String) -> emojis::Group {
    let emoji_char = emoji_str.chars().next().unwrap().to_string();
    emojis::get(emoji_char.as_str()).unwrap().group()
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
                input.scroll_into_view();
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

pub fn clear_cards(people: RwSignal<Vec<Person>>) {
    people.set(Vec::new())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_to_group_returns_smiley_group() {
        let group = emoji_to_group(String::from("😀 Smileys & Emotion"));
        assert_eq!(group, emojis::Group::SmileysAndEmotion);
    }

    #[test]
    fn test_emoji_to_group_returns_people_group() {
        let group = emoji_to_group(String::from("🤦 People & Body"));
        assert_eq!(group, emojis::Group::PeopleAndBody);
    }

    #[test]
    fn test_emoji_to_group_returns_animals_group() {
        let group = emoji_to_group(String::from("🐶 Animals & Nature"));
        assert_eq!(group, emojis::Group::AnimalsAndNature);
    }

    #[test]
    fn test_emoji_to_group_returns_food_group() {
        let group = emoji_to_group(String::from("🍇 Food & Drink"));
        assert_eq!(group, emojis::Group::FoodAndDrink);
    }

    #[test]
    fn test_emoji_to_group_returns_travel_group() {
        let group = emoji_to_group(String::from("✈️ Travel & Places"));
        assert_eq!(group, emojis::Group::TravelAndPlaces);
    }

    #[test]
    fn test_emoji_to_group_returns_activities_group() {
        let group = emoji_to_group(String::from("🎈 Activities"));
        assert_eq!(group, emojis::Group::Activities);
    }

    #[test]
    fn test_emoji_to_group_returns_objects_group() {
        let group = emoji_to_group(String::from("👘 Objects"));
        assert_eq!(group, emojis::Group::Objects);
    }

    #[test]
    fn test_emoji_to_group_returns_symbols_group() {
        let group = emoji_to_group(String::from("✅ Symbols"));
        assert_eq!(group, emojis::Group::Symbols);
    }

    #[test]
    fn test_emoji_to_group_returns_flags_group() {
        let group = emoji_to_group(String::from("🏁 Flags"));
        assert_eq!(group, emojis::Group::Flags);
    }
}
