use crate::primitives::*;
use leptos::prelude::*;

/// A context provider that wraps a Button component that triggers the collapsible menu.
#[component]
pub fn CollapsibleTrigger(children: Children) -> impl IntoView {
    view! { <TriggerPrimitive>{children()}</TriggerPrimitive> }
}
