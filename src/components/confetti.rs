use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use rand::Rng;
use std::time::Duration;
use web_sys::HtmlElement;

#[component]
pub fn Confetti() -> impl IntoView {
    let confetti_container = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        if let Some(container) = confetti_container.get() {
            // Create multiple confetti pieces
            for _ in 0..50 {
                let container_clone = container.clone();
                set_timeout(
                    move || create_confetti_piece(&container_clone),
                    Duration::from_millis(rand::thread_rng().gen_range(0..1000)),
                );
            }
        }
    });

    view! {
        <div
            node_ref=confetti_container
            style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 1000;"
        />
    }
}

pub fn start_confetti(confetti_container: NodeRef<leptos::html::Div>) {
    if let Some(container) = confetti_container.get() {
        // Create multiple confetti pieces
        for _ in 0..50 {
            let container_clone = container.clone();
            set_timeout(
                move || create_confetti_piece(&container_clone),
                Duration::from_millis(rand::thread_rng().gen_range(0..1000)),
            );
        }
    }
}

// Create a single piece of confetti
fn create_confetti_piece(container: &HtmlElement) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    let div = element.dyn_into::<web_sys::HtmlElement>().unwrap();

    let mut rng = rand::thread_rng();

    // Random initial position
    let x = rng.gen_range(0..window.inner_width().unwrap().as_f64().unwrap() as i32);

    // Random color
    let colors = [
        "#dc2626", "#a16207", "#15803d", "#1d4ed8", "#7e22ce", "#be185d",
    ];
    let color = colors[rng.gen_range(0..colors.len())];

    // Set styles
    div.style().set_property("position", "fixed").unwrap();
    div.style().set_property("width", "10px").unwrap();
    div.style().set_property("height", "10px").unwrap();
    div.style().set_property("background-color", color).unwrap();
    div.style()
        .set_property("left", &format!("{}px", x))
        .unwrap();
    div.style().set_property("top", "-10px").unwrap();

    container.append_child(&div).unwrap();

    // Animate the confetti piece
    let duration = rng.gen_range(1000..3000);
    let fall_distance = window.inner_height().unwrap().as_f64().unwrap() + 10.0;

    div.style()
        .set_property(
            "transition",
            &format!("top {}ms linear, transform {}ms linear", duration, duration),
        )
        .unwrap();

    // Clone the elements we need for the cleanup closure
    let div_clone = div.clone();
    let container_clone = container.clone();

    // Small delay to ensure the transition property is set
    set_timeout(
        move || {
            div.style()
                .set_property("top", &format!("{}px", fall_distance))
                .unwrap();
            div.style()
                .set_property(
                    "transform",
                    &format!("rotate({}deg)", rng.gen_range(0..360)),
                )
                .unwrap();

            // Use the cloned elements for cleanup
            set_timeout(
                move || {
                    let _ = container_clone.remove_child(&div_clone);
                },
                Duration::from_millis(duration as u64),
            );
        },
        Duration::from_millis(50),
    );
}
