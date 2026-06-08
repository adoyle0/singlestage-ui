mod checkbox;
mod group;

pub use checkbox::*;
pub use group::*;

use crate::Reactive;

#[derive(Clone)]
pub(crate) struct CheckboxGroupContext {
    pub name: String,
    pub disabled: Reactive<bool>,
    pub invalid: Reactive<bool>,
    pub value: Reactive<Vec<String>>,
}
