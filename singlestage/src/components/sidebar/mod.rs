mod content;
mod footer;
mod group;
mod header;
mod inset;
mod menu;
mod provider;
mod rail;
mod separator;
mod sidebar;
mod sub;
mod trigger;

pub use content::*;
pub use footer::*;
pub use group::*;
pub use header::*;
pub use inset::*;
pub use menu::*;
pub use provider::*;
pub use rail::*;
pub use separator::*;
pub use sidebar::*;
pub use sub::*;
pub use trigger::*;

use crate::Reactive;

#[derive(Clone)]
pub struct SidebarContext {
    pub open: Reactive<bool>,
    pub side: Reactive<String>,
}
