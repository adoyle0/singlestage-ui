mod opt_group;
mod option;
mod select;

pub use opt_group::*;
pub use option::*;
pub use select::*;

use crate::Reactive;

#[derive(Clone)]
pub(crate) struct SelectContext {
    value: Reactive<String>,
}
