use crate::TriggerContext;
use leptos::{context::Provider, prelude::*};

/// A button that opens the collapsible menu.
#[component]
pub fn CollapsibleTrigger(children: Children) -> impl IntoView {
    view! { <Provider value=TriggerContext {}>{children()}</Provider> }
}
