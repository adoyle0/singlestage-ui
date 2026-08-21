use crate::primitives::*;
use leptos::prelude::*;

/// A context provider that wraps a button that triggers the dialog.
#[component]
pub fn DialogTrigger(children: Children) -> impl IntoView {
    view! { <TriggerPrimitive>{children()}</TriggerPrimitive> }
}
