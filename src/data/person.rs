use leptos::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default)]
pub struct Person {
    pub id: Uuid,
    pub name: RwSignal<String>,
}
