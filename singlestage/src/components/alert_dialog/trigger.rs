use crate::primitives::*;
use leptos::prelude::*;

#[component]
pub fn AlertDialogTrigger(children: Children) -> impl IntoView {
    view! { <TriggerPrimitive>{children()}</TriggerPrimitive> }
}
