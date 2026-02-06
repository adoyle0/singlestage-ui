use crate::DialogActionContext;
use leptos::{context::Provider, prelude::*};

/// Wraps a Button that the user presses to acknowledge the dialog.
#[component]
pub fn DialogAction(children: Children) -> impl IntoView {
    view! { <Provider value=DialogActionContext {}>{children()}</Provider> }
}
