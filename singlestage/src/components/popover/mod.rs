mod content;
mod description;
mod header;
mod popover;
mod title;

pub use content::*;
pub use description::*;
pub use header::*;
pub use popover::*;
pub use title::*;

#[cfg(feature = "button")]
mod trigger;
#[cfg(feature = "button")]
pub use trigger::*;

use crate::Reactive;
use leptos::prelude::*;

#[derive(Clone)]
pub(crate) struct PopoverContext {
    pub modal: Reactive<bool>,
    pub menu_id: RwSignal<String>,
    pub open: Reactive<bool>,
    pub trigger_id: RwSignal<String>,
}
