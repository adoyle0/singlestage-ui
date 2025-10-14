mod checkbox;
mod group;

pub use checkbox::*;
pub use group::*;

use leptos::prelude::RwSignal;

#[derive(Clone)]
pub struct CheckboxGroupContext {
    invalid: RwSignal<bool>,
    value: RwSignal<Vec<String>>,
}
