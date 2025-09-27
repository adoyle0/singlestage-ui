use leptos::prelude::RwSignal;

mod group;
mod radio;

pub use group::*;
pub use radio::*;

#[derive(Clone)]
pub struct RadioGroupContext {
    name: String,
    invalid: RwSignal<bool>,
    value: RwSignal<String>,
}
