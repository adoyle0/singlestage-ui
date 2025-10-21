mod content;
mod popover;
mod trigger;

pub use content::*;
pub use popover::*;
pub use trigger::*;

use leptos::prelude::*;

#[derive(Clone)]
pub struct PopoverContext {
    menu_id: RwSignal<String>,
    trigger_id: RwSignal<String>,
}
