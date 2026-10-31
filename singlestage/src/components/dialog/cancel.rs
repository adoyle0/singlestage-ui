use crate::primitives::*;
use leptos::prelude::*;

/// A context provider that wraps Button that the user presses to dismiss the dialog.
#[component]
pub fn DialogCancel(children: Children) -> impl IntoView {
    view! { <DialogCancelPrimitive>{children()}</DialogCancelPrimitive> }
}
