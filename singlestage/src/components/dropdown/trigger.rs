use crate::primitives::*;
use leptos::prelude::*;

#[component]
pub fn DropdownMenuTrigger(children: Children) -> impl IntoView {
    view! { <TriggerPrimitive>{children()}</TriggerPrimitive> }
}
