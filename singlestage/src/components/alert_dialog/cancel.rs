use crate::primitives::*;
use leptos::prelude::*;

/// Wraps Button that the user presses to dismiss the dialog.
#[component]
pub fn AlertDialogCancel(children: Children) -> impl IntoView {
    view! { <DialogCancelPrimitive>{children()}</DialogCancelPrimitive> }
}
