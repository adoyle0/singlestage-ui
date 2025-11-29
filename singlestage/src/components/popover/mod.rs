mod content;
mod popover;
mod trigger;

pub use content::*;
pub use popover::*;
pub use trigger::*;

use leptos::prelude::*;

#[derive(Clone)]
pub struct PopoverContext {
    pub menu_id: RwSignal<String>,
    pub trigger_id: RwSignal<String>,
}

#[derive(Clone)]
pub struct PopoverTriggerContext {}
