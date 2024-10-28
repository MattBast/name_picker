use leptos::prelude::*;
use rand::seq::SliceRandom;
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

#[component]
pub fn NameCard(name: RwSignal<String>, emoji_list: Vec<String>) -> impl IntoView {
    let input_ref = ComponentRef::<InputRef>::new();

    // Create an effect that runs after the component is mounted
    Effect::new(move |_| {
        // Focus the input if it exists
        if let Some(input) = input_ref.get_untracked() {
            input.focus()
        }
    });

    view! {
        <div class="grow-0 shrink-0 basis-20 sm:basis-28 md:basis-40">
            <Card class="w-full h-full p-0 border-slate-200 border-2 rounded">
                // A picture to help identify the name.
                <CardPreview class="bg-blue-200 p-2.5">
                    <div class="flex items-center justify-center text-5xl hover:animate-bounce transition-all duration-75">
                        {random_emoji(emoji_list)}
                    </div>
                </CardPreview>
                // The editable name.
                <div class="p-2.5">
                    <Input value=name comp_ref=input_ref class="border-none text-center w-28 sm:w-40 md:w-52"/>
                </div>
            </Card>
        </div>
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

fn random_emoji(emoji_list: Vec<String>) -> String {
    emoji_list.choose(&mut rand::thread_rng()).unwrap().clone()
}
