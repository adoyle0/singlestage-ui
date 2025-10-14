mod checkbox;
mod group;

pub use checkbox::*;
pub use group::*;

use leptos::prelude::RwSignal;

#[derive(Clone)]
pub struct CheckboxGroupContext {
    pub invalid: RwSignal<bool>,
    pub value: RwSignal<Vec<String>>,
}
