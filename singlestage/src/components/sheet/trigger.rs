use crate::TriggerContext;
use leptos::{context::Provider, prelude::*};

#[component]
pub fn SheetTrigger(children: Children) -> impl IntoView {
    view! { <Provider value=TriggerContext {}>{children()}</Provider> }
}
