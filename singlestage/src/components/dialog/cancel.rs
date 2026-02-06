use crate::DialogCancelContext;
use leptos::{context::Provider, prelude::*};

/// Wraps Button that the user presses to dismiss the dialog.
#[component]
pub fn DialogCancel(children: Children) -> impl IntoView {
    view! { <Provider value=DialogCancelContext {}>{children()}</Provider> }
}
