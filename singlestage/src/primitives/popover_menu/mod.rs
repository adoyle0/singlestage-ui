mod content;
mod group;
mod item;
mod shortcut;
mod sub;

pub use content::*;
pub use group::*;
pub use item::*;
pub use shortcut::*;
pub use sub::*;

use crate::Reactive;
use leptos::prelude::*;

#[derive(Clone, Default)]
pub(crate) struct PopoverMenuContext {
    pub dismissable: Reactive<bool>,
    pub menu_id: RwSignal<String>,
    pub open: Reactive<bool>,
    pub trigger_id: RwSignal<String>,
}

#[derive(Clone)]
pub(crate) struct PopoverMenuGroupContext {
    pub heading_id: RwSignal<String>,
}

#[derive(Clone)]
pub(crate) struct MenuSubContext {
    pub menu_id: RwSignal<String>,
    pub open: Reactive<bool>,
    pub trigger_id: RwSignal<String>,
}
