mod content;
mod group;
mod item;
mod label;
mod menu;
mod separator;
mod shortcut;
mod sub;
mod trigger;

pub use content::*;
pub use group::*;
pub use item::*;
pub use label::*;
pub use menu::*;
pub use separator::*;
pub use shortcut::*;
pub use sub::*;
pub use trigger::*;

use crate::Reactive;
use leptos::prelude::*;

#[derive(Clone)]
pub struct ContextMenuContext {
    menu_id: RwSignal<String>,
    pub open: Reactive<bool>,
    x: RwSignal<i32>,
    y: RwSignal<i32>,
}

#[derive(Clone)]
pub struct ContextMenuGroupContext {
    heading_id: RwSignal<String>,
}
