use crate::components::NameCard;
use leptos::prelude::*;
use thaw::*;

#[component]
pub fn Home() -> impl IntoView {
    let names = RwSignal::new(Vec::new());
    names.update(|names| {
        names.push(RwSignal::new(String::from("Saburo")));
        names.push(RwSignal::new(String::from("Hanako")));
        names.push(RwSignal::new(String::from("Michiko")));
    });

    let emoji_list = get_emojis();

    view! {
        <div class="min-h-screen w-full flex items-center justify-center p-4">
            <CardGrid names emoji_list/>
        </div>
    }
}

#[component]
pub fn CardGrid(names: RwSignal<Vec<RwSignal<String>>>, emoji_list: Vec<String>) -> impl IntoView {
    view! {
        <Flex gap=FlexGap::Small justify=FlexJustify::Center class="flex-wrap">
            // Create one card for every name.
            <For each=move || names.get() key=|name| name.clone() let:name>
                <NameCard name emoji_list=emoji_list.clone()/>
            </For>
            // Button for adding a new card.
            <Button
                appearance=ButtonAppearance::Transparent
                icon=icondata::ChPlus
                on_click=move |_| names.update(|names| names.push(RwSignal::new(String::new())))
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
