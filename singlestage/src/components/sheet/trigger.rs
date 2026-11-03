use crate::primitives::*;
use leptos::prelude::*;

/// Wraps a button to be used to trigger opening the sheet.
#[component]
pub fn SheetTrigger(children: Children) -> impl IntoView {
    view! { <TriggerPrimitive>{children()}</TriggerPrimitive> }
}
