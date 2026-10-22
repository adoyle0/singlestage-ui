use crate::primitives::*;
use leptos::prelude::*;

#[component]
pub fn PopoverTrigger(children: Children) -> impl IntoView {
    view! { <TriggerPrimitive>{children()}</TriggerPrimitive> }
}
