use crate::primitives::*;
use leptos::prelude::*;

/// A button that opens the collapsible menu.
#[component]
pub fn CollapsibleTrigger(children: Children) -> impl IntoView {
    view! { <TriggerPrimitive>{children()}</TriggerPrimitive> }
}
