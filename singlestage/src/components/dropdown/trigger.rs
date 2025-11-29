use crate::{DropdownMenuContext, DropdownTriggerContext};
use leptos::{context::Provider, prelude::*};

/// Provides context to a `Button` that triggers a `DropdownMenu`.
#[component]
pub fn DropdownMenuTrigger(children: Children) -> impl IntoView {
    let menu = expect_context::<DropdownMenuContext>();

    let uuid = uuid::Uuid::new_v4();
    menu.trigger_id.set(uuid.to_string());

    let context = DropdownTriggerContext {};

    view! { <Provider value=context>{children()}</Provider> }
}
