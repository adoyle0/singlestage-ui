mod opt_group;
mod option;
mod select;

pub use opt_group::*;
pub use option::*;
pub use select::*;

use crate::Reactive;
use leptos::prelude::MaybeProp;

#[derive(Clone)]
pub(crate) struct SelectContext {
    multiple: MaybeProp<bool>,
    value: Reactive<String>,
}
