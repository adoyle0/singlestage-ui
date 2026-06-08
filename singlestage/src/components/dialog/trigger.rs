use crate::primitives::*;
use leptos::prelude::*;

#[component]
pub fn DialogTrigger(children: Children) -> impl IntoView {
    view! { <TriggerPrimitive>{children()}</TriggerPrimitive> }
}
