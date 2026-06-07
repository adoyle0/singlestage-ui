use crate::Reactive;

mod group;
mod radio;

pub use group::*;
pub use radio::*;

#[derive(Clone)]
pub(crate) struct RadioGroupContext {
    pub name: String,
    pub disabled: Reactive<bool>,
    pub invalid: Reactive<bool>,
    pub value: Reactive<String>,
}
