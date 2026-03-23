use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub struct TriggerContext {}

#[component]
pub fn TriggerPrimitive(children: Children) -> impl IntoView {
    view! { <Provider value=TriggerContext {}>{children()}</Provider> }.into_any()
}
