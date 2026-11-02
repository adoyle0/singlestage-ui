mod carousel;
mod content;
mod item;
mod next;
mod previous;

pub use carousel::*;
pub use content::*;
pub use item::*;
pub use next::*;
pub use previous::*;

use leptos::prelude::*;

#[derive(Clone)]
pub struct CarouselContext {
    pub current_item: RwSignal<i32>,
    pub num_items: RwSignal<i32>,
    pub ul_ref: NodeRef<leptos::html::Ul>,
}
