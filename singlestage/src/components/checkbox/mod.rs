mod checkbox;
mod group;

pub use checkbox::*;
pub use group::*;

use crate::Reactive;

#[derive(Clone)]
pub struct CheckboxGroupContext {
    pub invalid: Reactive<bool>,
    pub value: Reactive<Vec<String>>,
}
