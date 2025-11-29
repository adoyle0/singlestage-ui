mod content;
mod group;
mod item;
mod label;
mod menu;
mod separator;
mod shortcut;
mod trigger;

pub use content::*;
pub use group::*;
pub use item::*;
pub use label::*;
pub use menu::*;
pub use separator::*;
pub use shortcut::*;
pub use trigger::*;

use leptos::prelude::*;

#[derive(Clone)]
pub struct DropdownMenuContext {
    pub menu_id: RwSignal<String>,
    pub trigger_id: RwSignal<String>,
}

#[derive(Clone)]
pub struct DropdownTriggerContext {}

#[derive(Clone)]
pub struct DropdownMenuGroupContext {
    heading_id: RwSignal<String>,
}
