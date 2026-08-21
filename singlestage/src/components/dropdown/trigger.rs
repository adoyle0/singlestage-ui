use crate::primitives::*;
use leptos::prelude::*;

/// A context provider that wraps a button that triggers the dropdown menu.
#[component]
pub fn DropdownMenuTrigger(children: Children) -> impl IntoView {
    view! { <TriggerPrimitive>{children()}</TriggerPrimitive> }
}
