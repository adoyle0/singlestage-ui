use super::PopoverTriggerContext;
use leptos::{context::Provider, prelude::*};

/// Provides context to a `Button` that triggers a popover.
#[component]
pub fn PopoverTrigger(children: Children) -> impl IntoView {
    let context = PopoverTriggerContext {};

    view! { <Provider value=context>{children()}</Provider> }
}
