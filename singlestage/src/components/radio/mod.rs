use crate::Reactive;

mod group;
mod radio;

pub use group::*;
pub use radio::*;

#[derive(Clone)]
pub(crate) struct RadioGroupContext {
    pub name: String,
    pub invalid: Reactive<bool>,
    pub value: Reactive<String>,
}
