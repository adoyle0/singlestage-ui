use crate::{DialogContext, Reactive};
use leptos::{context::Provider, prelude::*};

/// Contains all the parts of a Dialog component.
#[component]
pub fn Dialog(
    children: Children,

    /// Reactive signal that can remotely control the open state of the popover **but is not
    /// coupled to the actual open state of the popover**
    #[prop(optional, into)]
    open: Reactive<bool>,
) -> impl IntoView {
    let context = DialogContext {
        alert: false,
        open,
        ..Default::default()
    };

    view! { <Provider value=context>{children()}</Provider> }
}
