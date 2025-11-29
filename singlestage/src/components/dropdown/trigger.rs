use crate::DropdownTriggerContext;
use leptos::{context::Provider, prelude::*};

/// Provides context to a `Button` that triggers a dropdown menu.
#[component]
pub fn DropdownMenuTrigger(children: Children) -> impl IntoView {
    let context = DropdownTriggerContext {};

    view! { <Provider value=context>{children()}</Provider> }
}
