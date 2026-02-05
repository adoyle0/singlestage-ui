mod checkbox;
mod content;
mod context_menu;
mod dropdown_menu;
mod group;
mod item;
mod radio;
mod shortcut;
mod sub;

pub use checkbox::*;
pub use content::*;
pub use context_menu::*;
pub use dropdown_menu::*;
pub use group::*;
pub use item::*;
pub use radio::*;
pub use shortcut::*;
pub use sub::*;

use crate::Reactive;
use leptos::prelude::*;

#[derive(Clone, Default)]
pub struct PopoverMenuContext {
    pub dismissable: Reactive<bool>,
    pub menu_id: RwSignal<String>,
    pub open: Reactive<bool>,
    pub trigger_id: RwSignal<String>,
}

#[derive(Clone)]
pub struct DropdownMenuContext {}

#[derive(Clone)]
pub struct PopoverMenuGroupContext {
    pub heading_id: RwSignal<String>,
}
