mod content;
mod group;
mod label;

pub use content::*;
pub use group::*;
pub use label::*;

use leptos::prelude::*;

#[derive(Clone)]
pub struct SidebarGroupContext {
    pub label_id: RwSignal<String>,
}
