mod content;
mod popover;
mod trigger;

pub use content::*;
pub use popover::*;
pub use trigger::*;

use crate::Reactive;
use leptos::prelude::*;

#[derive(Clone)]
pub struct PopoverContext {
    pub dismissable: bool,
    pub menu_id: RwSignal<String>,
    pub open: Reactive<bool>,
    pub trigger_id: RwSignal<String>,
}

#[derive(Clone)]
pub struct PopoverTriggerContext {}

#[component]
pub fn PopoverHeader(children: Children) -> impl IntoView {
    view! { <div class="singlestage-popover-header">{children()}</div> }
}

#[component]
pub fn PopoverTitle(children: Children) -> impl IntoView {
    view! { <h1 class="singlestage-popover-title">{children()}</h1> }
}

#[component]
pub fn PopoverDescription(children: Children) -> impl IntoView {
    view! { <p class="singlestage-popover-description">{children()}</p> }
}
