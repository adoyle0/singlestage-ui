use crate::primitives::*;
use leptos::prelude::*;

#[component]
pub fn SheetTrigger(children: Children) -> impl IntoView {
    view! { <TriggerPrimitive>{children()}</TriggerPrimitive> }
}
