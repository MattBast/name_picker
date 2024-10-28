use leptos::*;
use rand::seq::SliceRandom;
use thaw::*;

#[component]
pub fn Home() -> impl IntoView {
    let names = create_rw_signal(Vec::new());
    names.update(|names| {
        names.push(create_rw_signal(String::from("Saburo")));
        names.push(create_rw_signal(String::from("Hanaka")));
        names.push(create_rw_signal(String::from("Michiko")));
    });

    let emoji_list = get_emojis();

    view! {
        // Makes the page the size of the current window and centres all items.
        <div class="min-h-screen w-full flex items-center justify-center p-4">
            <CardGrid names emoji_list/>
        </div>
    }
}

#[component]
pub fn CardGrid(names: RwSignal<Vec<RwSignal<String>>>, emoji_list: Vec<String>) -> impl IntoView {
    view! {
        // Put the card in the centre of the window, place gaps between each
        // one and create new lines of cards if the screen gets too small.
        <div class="flex flex-wrap gap-4 justify-center">
            // Create one card for every name.
            <For each=move || names.get() key=|name| name.clone() let:name>
                <NameCard name emoji_list=emoji_list.clone()/>
            </For>
        </div>
    }
}

#[component]
pub fn NameCard(name: RwSignal<String>, emoji_list: Vec<String>) -> impl IntoView {
    view! {
        // Stop the card from shrinking
        <div class="grow-0 shrink-0 basis-20 sm:basis-28 md:basis-40">
            <Card class="w-full h-full">
                // A picture to help identify the name.
                <CardHeader slot>
                    <div class="flex items-center justify-center text-5xl hover:animate-bounce transition-all duration-75">
                        {random_emoji(emoji_list)}
                    </div>
                </CardHeader>
                // The editable name.
                <Input
                    value={name}
                    class="border-none text-center w-28 sm:w-40 md:w-52"
                />
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
