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

use leptos::{html::Menu, prelude::*};

#[derive(Clone)]
pub struct ContextMenuContext {
    menu_id: RwSignal<String>,
    menu_ref: RwSignal<Option<NodeRef<Menu>>>,
    x: RwSignal<i32>,
    y: RwSignal<i32>,
}

#[derive(Clone)]
pub struct ContextMenuGroupContext {
    heading_id: RwSignal<String>,
}
