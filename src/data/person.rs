use leptos::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default)]
pub struct Person {
    pub id: Uuid,
    pub name: RwSignal<String>,
    pub input_ref: NodeRef<leptos::html::Input>,
    pub picked: RwSignal<bool>,
    pub not_picked: RwSignal<bool>,
}
