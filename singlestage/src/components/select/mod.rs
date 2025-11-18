mod content;
mod item;
mod select;

pub use content::*;
pub use item::*;
pub use select::*;

use crate::Reactive;
use leptos::prelude::MaybeProp;

#[derive(Clone)]
pub struct SelectContext {
    placeholder: MaybeProp<String>,
    value: Reactive<String>,
}
