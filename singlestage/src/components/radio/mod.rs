use crate::Reactive;

mod group;
mod radio;

pub use group::*;
pub use radio::*;

#[derive(Clone)]
pub struct RadioGroupContext {
    name: String,
    invalid: Reactive<bool>,
    value: Reactive<String>,
}
