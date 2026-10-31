mod content;
mod tooltip;
mod trigger;

pub use content::*;
pub use tooltip::*;
pub use trigger::*;

use leptos::prelude::*;

#[derive(Clone)]
pub(crate) struct TooltipContext {
    open: RwSignal<bool>,
    trigger_id: RwSignal<String>,
}
